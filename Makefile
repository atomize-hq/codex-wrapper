# Keep only things under ./crates/ (like the Substrate repo Makefile pattern).
CRATES_ALL := $(shell find . -maxdepth 3 -type f -name Cargo.toml -exec dirname {} \; | sort -u)
CRATES := $(filter ./crates/%,$(CRATES_ALL))
CRATES := $(patsubst ./%,%,$(CRATES))

CRATE_CMD ?= tokei .

LOG_ROOT := target/crate-logs
DATE_DIR := $(shell date -u +%m-%-d-%y)
LOG_DIR := $(LOG_ROOT)/$(DATE_DIR)
RUN_TS := $(shell date -u +%Y%m%dT%H%M%SZ)
FINAL_LOG := $(LOG_DIR)/__all-crates.$(RUN_TS).log

.PHONY: tokei-all-crates
tokei-all-crates:
	@mkdir -p "$(LOG_DIR)"
	@echo "Date dir (UTC): $(DATE_DIR)"
	@echo "Run timestamp (UTC): $(RUN_TS)"
	@echo "Log dir: $(LOG_DIR)"
	@echo "CRATES = $(CRATES)"
	@set -e; \
	for d in $(CRATES); do \
	  crate=$$(basename "$$d"); \
	  cmd_tag=$$(printf '%s\n' "$(CRATE_CMD)" | tr ' /' '-_'); \
	  log="$(LOG_DIR)/$${crate}_$${cmd_tag}_$(RUN_TS).log"; \
	  echo "===== BEGIN $$d =====" | tee "$$log"; \
	  (cd "$$d" && $(CRATE_CMD)) 2>&1 | tee -a "$$log"; \
	  echo "===== END $$d =====" | tee -a "$$log"; \
	  echo "" >> "$$log"; \
	done; \
	cat "$(LOG_DIR)"/*_*$$(printf '%s\n' "$(RUN_TS)").log > "$(FINAL_LOG)"; \
	echo "Combined log written to: $(FINAL_LOG)"

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: check
check:
	cargo check --workspace --all-targets

.PHONY: test
test:
	cargo test --workspace --all-targets

.PHONY: flightcheck
flightcheck:
	@echo "##flightcheck -- must run from repo root"
	@echo "##flightcheck -- must pass for *integ tasks to be considered green"
	cargo fmt && cargo clippy --workspace --all-targets -- -D warnings && cargo clean && cargo check --workspace --all-targets && cargo test --workspace --all-targets

.PHONY: preflight
.PHONY: hygiene
hygiene:
	./scripts/check_repo_hygiene.sh

.PHONY: preflight
preflight: hygiene flightcheck
