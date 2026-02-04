use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::{
    models::{UnionArgV2, UnionCommandV2, UnionFlagV2, UnionSnapshotV2, WrapperCoverageV1},
    rules::{ParityExclusionUnit, RulesFile, RulesParityExclusions},
    util,
    wrapper::{self, CoverageResolution, FilterMode, WrapperIndex},
    ReportError,
};

pub(super) fn index_upstream(union: &UnionSnapshotV2) -> BTreeMap<Vec<String>, UnionCommandV2> {
    let mut out = BTreeMap::new();
    for cmd in &union.commands {
        out.insert(cmd.path.clone(), cmd.clone());
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_report(
    rules: &RulesFile,
    parity_exclusions: Option<&ParityExclusionsIndex>,
    version: &str,
    platform_mode: &str,
    target_triple: Option<&str>,
    filter_mode: FilterMode<'_>,
    report_targets: &[String],
    upstream: &BTreeMap<Vec<String>, UnionCommandV2>,
    wrapper: &WrapperCoverageV1,
    wrapper_index: &WrapperIndex,
    generated_at: &str,
) -> Result<CoverageReportV1, ReportError> {
    let report_target_set: BTreeSet<String> = report_targets.iter().cloned().collect();
    let expected_set: BTreeSet<String> = rules.union.expected_targets.iter().cloned().collect();
    let iu_roots = build_iu_roots(
        wrapper,
        wrapper_index,
        &report_target_set,
        &rules.union.expected_targets,
        filter_mode,
    )?;

    if matches!(filter_mode, FilterMode::All)
        && !expected_set.is_subset(&report_target_set)
        && rules.report.filter_semantics.when_union_incomplete.all == "error"
    {
        return Err(ReportError::Rules(
            "cannot generate platform_filter.mode=all with an incomplete union target set"
                .to_string(),
        ));
    }

    let mut missing_commands: Vec<ReportCommandDeltaV1> = Vec::new();
    let mut missing_flags: Vec<ReportFlagDeltaV1> = Vec::new();
    let mut missing_args: Vec<ReportArgDeltaV1> = Vec::new();

    let mut excluded_commands: Vec<ReportCommandDeltaV1> = Vec::new();
    let mut excluded_flags: Vec<ReportFlagDeltaV1> = Vec::new();
    let mut excluded_args: Vec<ReportArgDeltaV1> = Vec::new();

    let mut passthrough_candidates: Vec<ReportCommandDeltaV1> = Vec::new();
    let mut unsupported: Vec<ReportCommandDeltaV1> = Vec::new();
    let mut intentionally_unsupported: Vec<ReportIntentionallyUnsupportedDeltaV1> = Vec::new();
    let mut wrapper_only_commands: Vec<ReportCommandDeltaV1> = Vec::new();
    let mut wrapper_only_flags: Vec<ReportFlagDeltaV1> = Vec::new();
    let mut wrapper_only_args: Vec<ReportArgDeltaV1> = Vec::new();

    // Upstream → missing/unsupported/iu/passthrough
    for (path, cmd) in upstream {
        if !present_on_filter(
            &cmd.available_on,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
        ) {
            continue;
        }

        if let Some(ex) = parity_exclusions.and_then(|idx| idx.commands.get(path)) {
            let cmd_res = wrapper::resolve_wrapper(
                wrapper_index
                    .commands
                    .get(path)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
                "command",
                &format!("path={}", util::format_path(path)),
            )?;
            excluded_commands.push(ReportCommandDeltaV1 {
                path: path.clone(),
                upstream_available_on: cmd.available_on.clone(),
                wrapper_level: cmd_res.level.clone(),
                note: Some(ex.note.clone()),
            });
            continue;
        }

        let cmd_res = wrapper::resolve_wrapper(
            wrapper_index
                .commands
                .get(path)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
            "command",
            &format!("path={}", util::format_path(path)),
        )?;

        if cmd_res.level.is_none() {
            if let Some(root) = find_inherited_iu_root(
                &iu_roots,
                path,
                &cmd.available_on,
                &report_target_set,
                "command",
            )? {
                intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Command(
                    ReportCommandDeltaV1 {
                        path: path.clone(),
                        upstream_available_on: cmd.available_on.clone(),
                        wrapper_level: Some("intentionally_unsupported".to_string()),
                        note: Some(root.note.clone()),
                    },
                ));
            } else {
                classify_command_delta(
                    &mut missing_commands,
                    &mut passthrough_candidates,
                    &mut unsupported,
                    path,
                    &cmd.available_on,
                    &cmd_res,
                );
            }
        } else if cmd_res.level.as_deref() == Some("intentionally_unsupported") {
            let note = require_non_empty_note(
                cmd_res.note.as_deref(),
                "command",
                &format!("path={}", util::format_path(path)),
            )?;
            intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Command(
                ReportCommandDeltaV1 {
                    path: path.clone(),
                    upstream_available_on: cmd.available_on.clone(),
                    wrapper_level: Some("intentionally_unsupported".to_string()),
                    note: Some(note),
                },
            ));
        } else {
            classify_command_delta(
                &mut missing_commands,
                &mut passthrough_candidates,
                &mut unsupported,
                path,
                &cmd.available_on,
                &cmd_res,
            );
        }

        for flag in &cmd.flags {
            if !present_on_filter(
                &flag.available_on,
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
            ) {
                continue;
            }
            if let Some(ex) =
                parity_exclusions.and_then(|idx| idx.flags.get(&(path.clone(), flag.key.clone())))
            {
                let key = (path.clone(), flag.key.clone());
                let res = wrapper::resolve_wrapper(
                    wrapper_index
                        .flags
                        .get(&key)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]),
                    &report_target_set,
                    &rules.union.expected_targets,
                    filter_mode,
                    "flag",
                    &format!("path={} key={}", util::format_path(path), flag.key),
                )?;
                excluded_flags.push(ReportFlagDeltaV1 {
                    path: path.clone(),
                    key: flag.key.clone(),
                    upstream_available_on: flag.available_on.clone(),
                    wrapper_level: res.level.clone(),
                    note: Some(ex.note.clone()),
                });
                continue;
            }
            let key = (path.clone(), flag.key.clone());
            let res = wrapper::resolve_wrapper(
                wrapper_index
                    .flags
                    .get(&key)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
                "flag",
                &format!("path={} key={}", util::format_path(path), flag.key),
            )?;
            if res.level.is_none() {
                if let Some(root) = find_inherited_iu_root(
                    &iu_roots,
                    path,
                    &flag.available_on,
                    &report_target_set,
                    "flag",
                )? {
                    intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Flag(
                        ReportFlagDeltaV1 {
                            path: path.to_vec(),
                            key: flag.key.clone(),
                            upstream_available_on: flag.available_on.clone(),
                            wrapper_level: Some("intentionally_unsupported".to_string()),
                            note: Some(root.note.clone()),
                        },
                    ));
                } else {
                    classify_flag_delta(&mut missing_flags, path, flag, &res);
                }
            } else if res.level.as_deref() == Some("intentionally_unsupported") {
                let note = require_non_empty_note(
                    res.note.as_deref(),
                    "flag",
                    &format!("path={} key={}", util::format_path(path), flag.key),
                )?;
                intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Flag(
                    ReportFlagDeltaV1 {
                        path: path.to_vec(),
                        key: flag.key.clone(),
                        upstream_available_on: flag.available_on.clone(),
                        wrapper_level: Some("intentionally_unsupported".to_string()),
                        note: Some(note),
                    },
                ));
            } else {
                classify_flag_delta(&mut missing_flags, path, flag, &res);
            }
        }

        for arg in &cmd.args {
            if !present_on_filter(
                &arg.available_on,
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
            ) {
                continue;
            }
            if let Some(ex) =
                parity_exclusions.and_then(|idx| idx.args.get(&(path.clone(), arg.name.clone())))
            {
                let key = (path.clone(), arg.name.clone());
                let res = wrapper::resolve_wrapper(
                    wrapper_index
                        .args
                        .get(&key)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]),
                    &report_target_set,
                    &rules.union.expected_targets,
                    filter_mode,
                    "arg",
                    &format!("path={} name={}", util::format_path(path), arg.name),
                )?;
                excluded_args.push(ReportArgDeltaV1 {
                    path: path.clone(),
                    name: arg.name.clone(),
                    upstream_available_on: arg.available_on.clone(),
                    wrapper_level: res.level.clone(),
                    note: Some(ex.note.clone()),
                });
                continue;
            }
            let key = (path.clone(), arg.name.clone());
            let res = wrapper::resolve_wrapper(
                wrapper_index
                    .args
                    .get(&key)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
                "arg",
                &format!("path={} name={}", util::format_path(path), arg.name),
            )?;
            if res.level.is_none() {
                if let Some(root) = find_inherited_iu_root(
                    &iu_roots,
                    path,
                    &arg.available_on,
                    &report_target_set,
                    "arg",
                )? {
                    intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Arg(
                        ReportArgDeltaV1 {
                            path: path.to_vec(),
                            name: arg.name.clone(),
                            upstream_available_on: arg.available_on.clone(),
                            wrapper_level: Some("intentionally_unsupported".to_string()),
                            note: Some(root.note.clone()),
                        },
                    ));
                } else {
                    classify_arg_delta(&mut missing_args, path, arg, &res);
                }
            } else if res.level.as_deref() == Some("intentionally_unsupported") {
                let note = require_non_empty_note(
                    res.note.as_deref(),
                    "arg",
                    &format!("path={} name={}", util::format_path(path), arg.name),
                )?;
                intentionally_unsupported.push(ReportIntentionallyUnsupportedDeltaV1::Arg(
                    ReportArgDeltaV1 {
                        path: path.to_vec(),
                        name: arg.name.clone(),
                        upstream_available_on: arg.available_on.clone(),
                        wrapper_level: Some("intentionally_unsupported".to_string()),
                        note: Some(note),
                    },
                ));
            } else {
                classify_arg_delta(&mut missing_args, path, arg, &res);
            }
        }
    }

    // Wrapper → wrapper-only (relative to platform filter semantics)
    for (path, entries) in &wrapper_index.commands {
        let res = wrapper::resolve_wrapper(
            entries,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
            "command",
            &format!("path={}", util::format_path(path)),
        )?;
        if !res.present {
            continue;
        }

        let upstream_avail = upstream
            .get(path)
            .map(|c| c.available_on.clone())
            .unwrap_or_else(|| util::ordered_subset(&rules.union.expected_targets, &res.targets));
        let upstream_present = upstream.get(path).is_some_and(|c| {
            present_on_filter(
                &c.available_on,
                &report_target_set,
                &rules.union.expected_targets,
                filter_mode,
            )
        });

        if !upstream_present {
            wrapper_only_commands.push(ReportCommandDeltaV1 {
                path: path.clone(),
                upstream_available_on: upstream_avail,
                wrapper_level: res.level.clone(),
                note: res.note.clone(),
            });
        }
    }

    for ((path, key), entries) in &wrapper_index.flags {
        let res = wrapper::resolve_wrapper(
            entries,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
            "flag",
            &format!("path={} key={key}", util::format_path(path)),
        )?;
        if !res.present {
            continue;
        }
        let (upstream_avail, upstream_present) = upstream_flag_availability(
            upstream,
            path,
            key,
            &res,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
        );
        if !upstream_present {
            wrapper_only_flags.push(ReportFlagDeltaV1 {
                path: path.clone(),
                key: key.clone(),
                upstream_available_on: upstream_avail,
                wrapper_level: res.level.clone(),
                note: res.note.clone(),
            });
        }
    }

    for ((path, name), entries) in &wrapper_index.args {
        let res = wrapper::resolve_wrapper(
            entries,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
            "arg",
            &format!("path={} name={name}", util::format_path(path)),
        )?;
        if !res.present {
            continue;
        }
        let (upstream_avail, upstream_present) = upstream_arg_availability(
            upstream,
            path,
            name,
            &res,
            &report_target_set,
            &rules.union.expected_targets,
            filter_mode,
        );
        if !upstream_present {
            wrapper_only_args.push(ReportArgDeltaV1 {
                path: path.clone(),
                name: name.clone(),
                upstream_available_on: upstream_avail,
                wrapper_level: res.level.clone(),
                note: res.note.clone(),
            });
        }
    }

    missing_commands.sort_by(|a, b| util::cmp_path(&a.path, &b.path));
    missing_flags.sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.key.cmp(&b.key)));
    missing_args.sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.name.cmp(&b.name)));

    excluded_commands.sort_by(|a, b| util::cmp_path(&a.path, &b.path));
    excluded_flags.sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.key.cmp(&b.key)));
    excluded_args
        .sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.name.cmp(&b.name)));

    passthrough_candidates.sort_by(|a, b| util::cmp_path(&a.path, &b.path));
    unsupported.sort_by(|a, b| util::cmp_path(&a.path, &b.path));
    intentionally_unsupported.sort_by(cmp_iu_delta);

    wrapper_only_commands.sort_by(|a, b| util::cmp_path(&a.path, &b.path));
    wrapper_only_flags
        .sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.key.cmp(&b.key)));
    wrapper_only_args
        .sort_by(|a, b| util::cmp_path(&a.path, &b.path).then_with(|| a.name.cmp(&b.name)));

    let deltas = ReportDeltasV1 {
        missing_commands,
        missing_flags,
        missing_args,
        excluded_commands: if excluded_commands.is_empty() {
            None
        } else {
            Some(excluded_commands)
        },
        excluded_flags: if excluded_flags.is_empty() {
            None
        } else {
            Some(excluded_flags)
        },
        excluded_args: if excluded_args.is_empty() {
            None
        } else {
            Some(excluded_args)
        },
        passthrough_candidates: if passthrough_candidates.is_empty() {
            None
        } else {
            Some(passthrough_candidates)
        },
        unsupported: if unsupported.is_empty() {
            None
        } else {
            Some(unsupported)
        },
        intentionally_unsupported: if intentionally_unsupported.is_empty() {
            None
        } else {
            Some(intentionally_unsupported)
        },
        wrapper_only_commands: if wrapper_only_commands.is_empty() {
            None
        } else {
            Some(wrapper_only_commands)
        },
        wrapper_only_flags: if wrapper_only_flags.is_empty() {
            None
        } else {
            Some(wrapper_only_flags)
        },
        wrapper_only_args: if wrapper_only_args.is_empty() {
            None
        } else {
            Some(wrapper_only_args)
        },
    };

    Ok(CoverageReportV1 {
        schema_version: 1,
        generated_at: generated_at.to_string(),
        inputs: ReportInputsV1 {
            upstream: ReportUpstreamInputsV1 {
                semantic_version: version.to_string(),
                mode: "union".to_string(),
                targets: report_targets.to_vec(),
            },
            wrapper: ReportWrapperInputsV1 {
                schema_version: wrapper.schema_version,
                wrapper_version: wrapper.wrapper_version.clone(),
            },
            rules: ReportRulesInputsV1 {
                rules_schema_version: rules.rules_schema_version,
            },
        },
        platform_filter: PlatformFilterV1 {
            mode: platform_mode.to_string(),
            target_triple: target_triple.map(ToString::to_string),
        },
        deltas,
    })
}

fn present_on_filter(
    available_on: &[String],
    report_targets: &BTreeSet<String>,
    expected_targets: &[String],
    mode: FilterMode<'_>,
) -> bool {
    match mode {
        FilterMode::Any => available_on.iter().any(|t| report_targets.contains(t)),
        FilterMode::ExactTarget(t) => available_on.iter().any(|x| x == t),
        FilterMode::All => expected_targets
            .iter()
            .all(|t| available_on.iter().any(|x| x == t)),
    }
}

fn classify_command_delta(
    missing: &mut Vec<ReportCommandDeltaV1>,
    passthrough_candidates: &mut Vec<ReportCommandDeltaV1>,
    unsupported: &mut Vec<ReportCommandDeltaV1>,
    path: &[String],
    upstream_available_on: &[String],
    wrapper: &CoverageResolution,
) {
    let entry = ReportCommandDeltaV1 {
        path: path.to_vec(),
        upstream_available_on: upstream_available_on.to_vec(),
        wrapper_level: wrapper.level.clone(),
        note: wrapper.note.clone(),
    };

    match wrapper.level.as_deref() {
        None => missing.push(entry),
        Some("unknown") => missing.push(entry),
        Some("unsupported") => unsupported.push(entry),
        Some("intentionally_unsupported") => {}
        Some("passthrough") => passthrough_candidates.push(entry),
        Some("explicit") => {}
        Some(other) => missing.push(ReportCommandDeltaV1 {
            wrapper_level: Some(other.to_string()),
            ..entry
        }),
    }
}

fn classify_flag_delta(
    out: &mut Vec<ReportFlagDeltaV1>,
    path: &[String],
    flag: &UnionFlagV2,
    wrapper: &CoverageResolution,
) {
    match wrapper.level.as_deref() {
        None | Some("unknown") | Some("unsupported") => out.push(ReportFlagDeltaV1 {
            path: path.to_vec(),
            key: flag.key.clone(),
            upstream_available_on: flag.available_on.clone(),
            wrapper_level: wrapper.level.clone(),
            note: wrapper.note.clone(),
        }),
        Some("intentionally_unsupported") => {}
        Some("explicit") | Some("passthrough") => {}
        Some(other) => out.push(ReportFlagDeltaV1 {
            path: path.to_vec(),
            key: flag.key.clone(),
            upstream_available_on: flag.available_on.clone(),
            wrapper_level: Some(other.to_string()),
            note: wrapper.note.clone(),
        }),
    }
}

fn classify_arg_delta(
    out: &mut Vec<ReportArgDeltaV1>,
    path: &[String],
    arg: &UnionArgV2,
    wrapper: &CoverageResolution,
) {
    match wrapper.level.as_deref() {
        None | Some("unknown") | Some("unsupported") => out.push(ReportArgDeltaV1 {
            path: path.to_vec(),
            name: arg.name.clone(),
            upstream_available_on: arg.available_on.clone(),
            wrapper_level: wrapper.level.clone(),
            note: wrapper.note.clone(),
        }),
        Some("intentionally_unsupported") => {}
        Some("explicit") | Some("passthrough") => {}
        Some(other) => out.push(ReportArgDeltaV1 {
            path: path.to_vec(),
            name: arg.name.clone(),
            upstream_available_on: arg.available_on.clone(),
            wrapper_level: Some(other.to_string()),
            note: wrapper.note.clone(),
        }),
    }
}

fn upstream_flag_availability(
    upstream: &BTreeMap<Vec<String>, UnionCommandV2>,
    path: &[String],
    key: &str,
    wrapper_res: &CoverageResolution,
    report_targets: &BTreeSet<String>,
    expected_targets: &[String],
    mode: FilterMode<'_>,
) -> (Vec<String>, bool) {
    if let Some(cmd) = upstream.get(path) {
        if let Some(flag) = cmd.flags.iter().find(|f| f.key == key) {
            let present =
                present_on_filter(&flag.available_on, report_targets, expected_targets, mode);
            return (flag.available_on.clone(), present);
        }
        return (cmd.available_on.clone(), false);
    }
    (
        util::ordered_subset(expected_targets, &wrapper_res.targets),
        false,
    )
}

fn upstream_arg_availability(
    upstream: &BTreeMap<Vec<String>, UnionCommandV2>,
    path: &[String],
    name: &str,
    wrapper_res: &CoverageResolution,
    report_targets: &BTreeSet<String>,
    expected_targets: &[String],
    mode: FilterMode<'_>,
) -> (Vec<String>, bool) {
    if let Some(cmd) = upstream.get(path) {
        if let Some(arg) = cmd.args.iter().find(|a| a.name == name) {
            let present =
                present_on_filter(&arg.available_on, report_targets, expected_targets, mode);
            return (arg.available_on.clone(), present);
        }
        return (cmd.available_on.clone(), false);
    }
    (
        util::ordered_subset(expected_targets, &wrapper_res.targets),
        false,
    )
}

#[derive(Debug, Serialize)]
pub(super) struct CoverageReportV1 {
    schema_version: u32,
    generated_at: String,
    inputs: ReportInputsV1,
    platform_filter: PlatformFilterV1,
    deltas: ReportDeltasV1,
}

#[derive(Debug, Serialize)]
struct ReportInputsV1 {
    upstream: ReportUpstreamInputsV1,
    wrapper: ReportWrapperInputsV1,
    rules: ReportRulesInputsV1,
}

#[derive(Debug, Serialize)]
struct ReportUpstreamInputsV1 {
    semantic_version: String,
    mode: String,
    targets: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ReportWrapperInputsV1 {
    schema_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_version: Option<String>,
}

#[derive(Debug, Serialize)]
struct ReportRulesInputsV1 {
    rules_schema_version: u32,
}

#[derive(Debug, Serialize)]
struct PlatformFilterV1 {
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_triple: Option<String>,
}

#[derive(Debug, Serialize)]
struct ReportDeltasV1 {
    missing_commands: Vec<ReportCommandDeltaV1>,
    missing_flags: Vec<ReportFlagDeltaV1>,
    missing_args: Vec<ReportArgDeltaV1>,

    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_commands: Option<Vec<ReportCommandDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_flags: Option<Vec<ReportFlagDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_args: Option<Vec<ReportArgDeltaV1>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    passthrough_candidates: Option<Vec<ReportCommandDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unsupported: Option<Vec<ReportCommandDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intentionally_unsupported: Option<Vec<ReportIntentionallyUnsupportedDeltaV1>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_only_commands: Option<Vec<ReportCommandDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_only_flags: Option<Vec<ReportFlagDeltaV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_only_args: Option<Vec<ReportArgDeltaV1>>,
}

#[derive(Debug, Serialize)]
struct ReportCommandDeltaV1 {
    path: Vec<String>,
    upstream_available_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Debug, Serialize)]
struct ReportFlagDeltaV1 {
    path: Vec<String>,
    key: String,
    upstream_available_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Debug, Serialize)]
struct ReportArgDeltaV1 {
    path: Vec<String>,
    name: String,
    upstream_available_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrapper_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ReportIntentionallyUnsupportedDeltaV1 {
    Command(ReportCommandDeltaV1),
    Flag(ReportFlagDeltaV1),
    Arg(ReportArgDeltaV1),
}

#[derive(Debug, Clone)]
struct IuRoot {
    path: Vec<String>,
    targets: BTreeSet<String>,
    note: String,
}

fn build_iu_roots(
    wrapper: &WrapperCoverageV1,
    wrapper_index: &WrapperIndex,
    report_target_set: &BTreeSet<String>,
    expected_targets: &[String],
    filter_mode: FilterMode<'_>,
) -> Result<Vec<IuRoot>, ReportError> {
    let mut unique_paths: BTreeSet<Vec<String>> = BTreeSet::new();
    for cmd in &wrapper.coverage {
        if cmd.level == "intentionally_unsupported" {
            unique_paths.insert(cmd.path.clone());
        }
    }

    let mut roots = Vec::new();
    for path in unique_paths {
        let res = wrapper::resolve_wrapper(
            wrapper_index
                .commands
                .get(&path)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
            report_target_set,
            expected_targets,
            filter_mode,
            "command",
            &format!("path={}", util::format_path(&path)),
        )?;
        if res.level.as_deref() != Some("intentionally_unsupported") {
            continue;
        }
        let note = require_non_empty_note(
            res.note.as_deref(),
            "command",
            &format!("path={}", util::format_path(&path)),
        )?;
        roots.push(IuRoot {
            path,
            targets: res.targets,
            note,
        });
    }

    roots.sort_by(|a, b| {
        b.path
            .len()
            .cmp(&a.path.len())
            .then_with(|| util::cmp_path(&a.path, &b.path))
    });
    Ok(roots)
}

fn find_inherited_iu_root<'a>(
    roots: &'a [IuRoot],
    unit_path: &[String],
    unit_available_on: &[String],
    report_target_set: &BTreeSet<String>,
    unit_kind: &'static str,
) -> Result<Option<&'a IuRoot>, ReportError> {
    let relevant_targets: BTreeSet<String> = unit_available_on
        .iter()
        .filter(|t| report_target_set.contains(*t))
        .cloned()
        .collect();
    if relevant_targets.is_empty() {
        return Ok(None);
    }

    for root in roots {
        if !util::is_prefix(&root.path, unit_path) {
            continue;
        }

        let overlap: BTreeSet<String> = relevant_targets
            .intersection(&root.targets)
            .cloned()
            .collect();
        if overlap.is_empty() {
            continue;
        }
        if overlap != relevant_targets {
            return Err(ReportError::WrapperResolution {
                unit: unit_kind.to_string(),
                detail: format!(
                    "IU subtree root scope mismatch: root_path={} does not cover all upstream targets for unit_path={} (root_targets={} unit_targets={})",
                    util::format_path(&root.path),
                    util::format_path(unit_path),
                    root.targets.iter().cloned().collect::<Vec<_>>().join(","),
                    relevant_targets
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            });
        }

        return Ok(Some(root));
    }

    Ok(None)
}

fn require_non_empty_note(
    note: Option<&str>,
    unit_kind: &'static str,
    detail: &str,
) -> Result<String, ReportError> {
    match note.map(str::trim).filter(|s| !s.is_empty()) {
        Some(v) => Ok(v.to_string()),
        None => Err(ReportError::WrapperResolution {
            unit: unit_kind.to_string(),
            detail: format!("{detail} intentionally_unsupported requires non-empty note"),
        }),
    }
}

fn iu_kind_rank(entry: &ReportIntentionallyUnsupportedDeltaV1) -> u8 {
    match entry {
        ReportIntentionallyUnsupportedDeltaV1::Command(_) => 0,
        ReportIntentionallyUnsupportedDeltaV1::Flag(_) => 1,
        ReportIntentionallyUnsupportedDeltaV1::Arg(_) => 2,
    }
}

fn iu_path(entry: &ReportIntentionallyUnsupportedDeltaV1) -> &[String] {
    match entry {
        ReportIntentionallyUnsupportedDeltaV1::Command(v) => &v.path,
        ReportIntentionallyUnsupportedDeltaV1::Flag(v) => &v.path,
        ReportIntentionallyUnsupportedDeltaV1::Arg(v) => &v.path,
    }
}

fn cmp_iu_delta(
    a: &ReportIntentionallyUnsupportedDeltaV1,
    b: &ReportIntentionallyUnsupportedDeltaV1,
) -> std::cmp::Ordering {
    iu_kind_rank(a).cmp(&iu_kind_rank(b)).then_with(|| {
        util::cmp_path(iu_path(a), iu_path(b)).then_with(|| match (a, b) {
            (
                ReportIntentionallyUnsupportedDeltaV1::Flag(a),
                ReportIntentionallyUnsupportedDeltaV1::Flag(b),
            ) => a.key.cmp(&b.key),
            (
                ReportIntentionallyUnsupportedDeltaV1::Arg(a),
                ReportIntentionallyUnsupportedDeltaV1::Arg(b),
            ) => a.name.cmp(&b.name),
            _ => std::cmp::Ordering::Equal,
        })
    })
}

#[derive(Debug)]
pub(super) struct ParityExclusionsIndex {
    commands: BTreeMap<Vec<String>, ParityExclusionUnit>,
    flags: BTreeMap<(Vec<String>, String), ParityExclusionUnit>,
    args: BTreeMap<(Vec<String>, String), ParityExclusionUnit>,
}

pub(super) fn build_parity_exclusions_index(
    exclusions: &RulesParityExclusions,
) -> ParityExclusionsIndex {
    let mut commands = BTreeMap::new();
    let mut flags = BTreeMap::new();
    let mut args = BTreeMap::new();

    for unit in &exclusions.units {
        match unit.unit.as_str() {
            "command" => {
                commands.insert(unit.path.clone(), unit.clone());
            }
            "flag" => {
                if let Some(key) = unit.key.as_ref() {
                    flags.insert((unit.path.clone(), key.clone()), unit.clone());
                }
            }
            "arg" => {
                if let Some(name) = unit.name.as_ref() {
                    args.insert((unit.path.clone(), name.clone()), unit.clone());
                }
            }
            _ => {}
        }
    }

    ParityExclusionsIndex {
        commands,
        flags,
        args,
    }
}
