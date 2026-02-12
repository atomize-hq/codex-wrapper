use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Parser)]
pub struct Args {
    /// Root parity directory (e.g. cli_manifests/claude_code).
    #[arg(long)]
    pub root: PathBuf,

    /// Upstream semantic version to scaffold from (e.g. 2.1.29).
    #[arg(long)]
    pub version: String,

    /// Output feature directory under docs/project_management/next/.
    #[arg(long)]
    pub out: PathBuf,
}

#[derive(Debug, Error)]
pub enum ScaffoldError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing required file: {0}")]
    Missing(PathBuf),
    #[error("template missing required file: {0}")]
    TemplateMissing(PathBuf),
    #[error("invalid output path (must be under docs/project_management/next): {0}")]
    InvalidOut(PathBuf),
}

#[derive(Debug, Deserialize)]
struct CoverageAny {
    deltas: Deltas,
}

#[derive(Debug, Deserialize)]
struct Deltas {
    missing_commands: Vec<MissingCommand>,
    missing_flags: Vec<MissingFlag>,
    missing_args: Vec<MissingArg>,
}

#[derive(Debug, Deserialize)]
struct MissingCommand {
    path: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MissingFlag {
    path: Vec<String>,
    key: String,
}

#[derive(Debug, Deserialize)]
struct MissingArg {
    path: Vec<String>,
    name: String,
}

pub fn run(args: Args) -> Result<(), ScaffoldError> {
    let template_root = PathBuf::from("docs/project_management/next/_TEMPLATE_feature");
    if !template_root.is_dir() {
        return Err(ScaffoldError::TemplateMissing(template_root));
    }

    if !args
        .out
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/")
        .contains("docs/project_management/next")
    {
        return Err(ScaffoldError::InvalidOut(args.out));
    }

    let report_path = args
        .root
        .join("reports")
        .join(&args.version)
        .join("coverage.any.json");
    if !report_path.is_file() {
        return Err(ScaffoldError::Missing(report_path));
    }
    let report: CoverageAny = serde_json::from_slice(&fs::read(&report_path)?)?;

    fs::create_dir_all(&args.out)?;
    fs::create_dir_all(args.out.join("kickoff_prompts"))?;

    let feature_name = args
        .out
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("feature")
        .to_string();
    let feature_prefix = sanitize_branch_token(&feature_name);
    let scope = format!("v{}", args.version.replace('.', "-"));

    // plan.md
    {
        let template = read_template(&template_root.join("plan.md"))?;
        let mut text = template.replace("<FEATURE NAME>", &format!("{feature_name} – Plan"));
        text = text.replace("<feature>", &feature_name);
        text = text.replace("<feature-prefix>", &feature_prefix);
        text = text.replace("<scope>", &scope);
        text.push_str("\n\n## Parity Inputs\n");
        text.push_str(&format!(
            "- Parity root: `{}`\n- Version: `{}`\n- Coverage report: `{}`\n",
            args.root.display(),
            args.version,
            report_path.display()
        ));
        fs::write(args.out.join("plan.md"), text)?;
    }

    // session_log.md
    {
        let template = read_template(&template_root.join("session_log.md"))?;
        fs::write(args.out.join("session_log.md"), template)?;
    }

    // C0-spec.md (generated, not templated)
    {
        let mut spec = String::new();
        spec.push_str(&format!(
            "# C0-spec – Parity update for `{}` `{}`\n\n",
            args.root
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("parity-root"),
            args.version
        ));
        spec.push_str("## Scope\n");
        spec.push_str("- Use the generated coverage report as the work queue.\n");
        spec.push_str(&format!("- Report: `{}`\n", report_path.display()));
        spec.push_str("- Implement wrapper support or explicitly waive with `intentionally_unsupported` notes.\n");
        spec.push_str("- Regenerate artifacts and pass `codex-validate` for the parity root.\n\n");

        spec.push_str("### Missing commands\n");
        spec.push_str(&format_paths(
            &report
                .deltas
                .missing_commands
                .iter()
                .map(|c| &c.path)
                .collect::<Vec<_>>(),
        ));
        spec.push_str("\n### Missing flags\n");
        spec.push_str(&format_flag_identities(&report.deltas.missing_flags));
        spec.push_str("\n### Missing args\n");
        spec.push_str(&format_arg_identities(&report.deltas.missing_args));

        spec.push_str("\n## Acceptance Criteria\n");
        spec.push_str("- Wrapper changes address C0 scope.\n");
        spec.push_str("- Artifacts regenerated deterministically.\n");
        spec.push_str("- `cargo run -p xtask -- codex-validate --root <root>` passes.\n\n");
        spec.push_str("## Out of Scope\n");
        spec.push_str("- Promotion (pointer/current.json updates) unless explicitly requested.\n");

        fs::write(args.out.join("C0-spec.md"), spec)?;
    }

    // tasks.json
    {
        let template = read_template(&template_root.join("tasks.json"))?;
        let mut text = template.replace("<feature>", &feature_name);
        text = text.replace("<feature-prefix>", &feature_prefix);
        text = text.replace("<scope>", &scope);
        fs::write(args.out.join("tasks.json"), text)?;
    }

    // kickoff prompts
    {
        for name in ["C0-code.md", "C0-test.md", "C0-integ.md"] {
            let template = read_template(&template_root.join("kickoff_prompts").join(name))?;
            let mut text = template.replace("<feature>", &feature_name);
            text = text.replace("<feature-prefix>", &feature_prefix);
            text = text.replace("<scope>", &scope);
            text.push_str("\n\n## Parity Work Queue (from coverage.any.json)\n");
            text.push_str(&format!("Report: `{}`\n\n", report_path.display()));
            text.push_str("### Missing commands\n");
            text.push_str(&format_paths(
                &report
                    .deltas
                    .missing_commands
                    .iter()
                    .map(|c| &c.path)
                    .collect::<Vec<_>>(),
            ));
            text.push_str("\n### Missing flags\n");
            text.push_str(&format_flag_identities(&report.deltas.missing_flags));
            text.push_str("\n### Missing args\n");
            text.push_str(&format_arg_identities(&report.deltas.missing_args));
            fs::write(args.out.join("kickoff_prompts").join(name), text)?;
        }
    }

    // README.md (copied)
    {
        let template = read_template(&template_root.join("README.md"))?;
        fs::write(args.out.join("README.md"), template)?;
    }

    Ok(())
}

fn read_template(path: &Path) -> Result<String, ScaffoldError> {
    if !path.is_file() {
        return Err(ScaffoldError::TemplateMissing(path.to_path_buf()));
    }
    Ok(fs::read_to_string(path)?)
}

fn sanitize_branch_token(s: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in s.chars() {
        let ok = ch.is_ascii_alphanumeric() || ch == '-' || ch == '_';
        let mapped = if ok { ch } else { '-' };
        if mapped == '-' {
            if prev_dash {
                continue;
            }
            prev_dash = true;
        } else {
            prev_dash = false;
        }
        out.push(mapped.to_ascii_lowercase());
    }
    out.trim_matches('-').to_string()
}

fn format_paths(paths: &[&Vec<String>]) -> String {
    if paths.is_empty() {
        return "- (none)\n".to_string();
    }
    let mut out = String::new();
    for p in paths {
        out.push_str("- `");
        out.push_str(&format_path(p));
        out.push_str("`\n");
    }
    out
}

fn format_flag_identities(flags: &[MissingFlag]) -> String {
    if flags.is_empty() {
        return "- (none)\n".to_string();
    }
    let mut out = String::new();
    for f in flags {
        out.push_str("- `");
        out.push_str(&format!("{} {}", format_path(&f.path), f.key));
        out.push_str("`\n");
    }
    out
}

fn format_arg_identities(args: &[MissingArg]) -> String {
    if args.is_empty() {
        return "- (none)\n".to_string();
    }
    let mut out = String::new();
    for a in args {
        out.push_str("- `");
        out.push_str(&format!("{} {}", format_path(&a.path), a.name));
        out.push_str("`\n");
    }
    out
}

fn format_path(path: &[String]) -> String {
    if path.is_empty() {
        "<root>".to_string()
    } else {
        path.join(" ")
    }
}
