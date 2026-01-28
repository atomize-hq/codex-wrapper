#!/usr/bin/env bash
set -euo pipefail

enabled_features=()
while [[ "${1:-}" == "--enable" ]]; do
  enabled_features+=("${2:-}")
  shift 2
done

if [[ "${1:-}" == "--version" ]]; then
  echo "codex 0.77.0"
  exit 0
fi

if [[ "${1:-}" == "features" && "${2:-}" == "list" ]]; then
  cat <<'EOF'
base_feature stable true
extra_feature experimental false
EOF
  exit 0
fi

root_help() {
  local extra_line=""
  if printf '%s\n' "${enabled_features[@]}" | grep -qx "extra_feature"; then
    extra_line=$'  extra    Extra command (feature gated)\n'
  fi

  cat <<EOF
codex 0.77.0

Usage: codex [OPTIONS] <COMMAND>

Commands:
  zed      Zed command with a wrapped description
           that should not be interpreted as a new command token
${extra_line}  features  Inspect feature flags
  exec     Execute commands
  alpha    Alpha command
  help     Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet        Suppress output
  --json             Emit JSON
  -v                 Verbose output
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  root_help
  exit 0
fi

if [[ "${1:-}" != "help" ]]; then
  echo "fake_codex: unsupported invocation: $*" >&2
  exit 2
fi
shift

case "$*" in
  "")
    root_help
    ;;
  "help")
    cat <<'EOF'
Usage: codex help [COMMAND]...

Print this message or the help of the given subcommand(s)
EOF
    ;;
  "features")
    cat <<'EOF'
Inspect feature flags

Usage: codex features [OPTIONS] <COMMAND>

Commands:
  list  List known features
  help  Print this message or the help of the given subcommand(s)

Options:
  --enable <FEATURE>   Enable a feature (repeatable)
  --disable <FEATURE>  Disable a feature (repeatable)
  -h, --help           Print help
EOF
    ;;
  "features list")
    cat <<'EOF'
List known features

Usage: codex features list [OPTIONS]

Options:
  -h, --help  Print help
EOF
    ;;
  "features help")
    cat <<'EOF'
Print this message or the help of the given subcommand(s)

Usage: codex features help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
EOF
    ;;
  "exec")
    cat <<'EOF'
Usage: codex exec [OPTIONS] [PROMPT] [COMMAND]

Commands:
  start    Start execution
  resume   Resume execution

Options:
  --beta            Beta option (long only)

  -a, --alpha       Alpha option
  -c               Short-only option
EOF
    ;;
  "exec start")
    cat <<'EOF'
Usage: codex exec start [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>
          First line of prompt description
          Second line of prompt description (wrapped)

Options:
  --zulu            Zulu mode (long only)
  -b                Short-only option
  -a, --alpha       Alpha option
  -d, --delta PATH  Delta path (takes value)
EOF
    ;;
  "exec resume")
    cat <<'EOF'
Usage: codex exec resume [OPTIONS]

Options:
  --json            Emit JSON
  -q, --quiet       Suppress output
EOF
    ;;
  "alpha")
    cat <<'EOF'
Usage: codex alpha [OPTIONS]

Options:
  -x, --xray        Xray mode
EOF
    ;;
  "zed")
    cat <<'EOF'
Usage: codex zed [OPTIONS]

Options:
  --zebra           Zebra mode
EOF
    ;;
  "sandbox")
    cat <<'EOF'
Usage: codex sandbox [OPTIONS]

Options:
  --linux-only      Linux only
EOF
    ;;
  "extra")
    cat <<'EOF'
Extra command (feature gated)

Usage: codex extra [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input to process

Options:
  -h, --help  Print help
EOF
    ;;
  *)
    echo "fake_codex: unsupported help path: $*" >&2
    exit 2
    ;;
esac
