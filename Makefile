.DEFAULT_GOAL := help

PYTHON_INSTALL_EDITABLE ?= true# Whether to install python library in editable mode
PYTHON_INSTALL_FROM_DIST ?= ""# Path to python wheel to install from dist directory
BUILD_PROFILE ?= release
PYTHON_INTERPRETER ?= $(CURDIR)/bindings/python/.venv/bin/python
PYTHON_COMPARE_REPETITIONS ?= 10

RUST_SRC_DIRECTORY = $(CURDIR)/fast_aug
PYTHON_SRC_DIRECTORY = $(CURDIR)/bindings/python


.PHONY: help
help:  ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


.PHONY: build
build: build-rust build-python  ## Build all targets

.PHONY: build-rust
build-rust:  ## Build rust library
	@echo "Building rust library..."
	cd $(RUST_SRC_DIRECTORY) && cargo build --timings --profile $(BUILD_PROFILE)

.PHONY: build-python
build-python:  ## Build python library to (and install if PYTHON_INSTALL_FROM_DIST is set)
	@echo "Building python library..."
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip uninstall -y fast-aug
	@if [ -n "$(PYTHON_INSTALL_FROM_DIST)" ]; then \
		echo "-> Installing python library from dist directory <$(PYTHON_INSTALL_FROM_DIST)>..."; \
		$(PYTHON_INTERPRETER) -m pip install --upgrade --no-index --find-links=$(PYTHON_INSTALL_FROM_DIST) fast-aug; \
		$(PYTHON_INTERPRETER) -m pip install --find-links=$(PYTHON_INSTALL_FROM_DIST) fast-aug[test,compare]; \
	elif [ "$(PYTHON_INSTALL_EDITABLE)" = "true" ]; then \
		echo "-> Installing python library in editable mode..."; \
		cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install -v -e .[test,compare] --config-settings=build-args='--profile $(BUILD_PROFILE)' ; \
	else \
		echo "-> Building python wheel and installing..."; \
		cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip wheel . --no-deps -w dist --config-settings=build-args='--profile $(BUILD_PROFILE)'; \
		cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install --upgrade --no-index --find-links=./dist fast-aug; \
		cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install --find-links=./dist fast-aug[test,compare]; \
	fi


.PHONY: generate-stubs
generate-stubs:  ## Generate python stubs
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) generate_stubs.py


.PHONY: test
test: test-rust test-python  ## Run all tests

.PHONY: test-rust
test-rust:  ## Run rust tests
	@echo "Running rust tests..."
	cd $(RUST_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)

.PHONY: test-python
test-python: build-python  ## Run python tests
	@echo "Running python tests..."
	cd $(PYTHON_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install maturin
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) generate_stubs.py --check
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pytest tests/


.PHONY: format
format: format-rust format-python  ## Format all code

.PHONY: format-rust
format-rust:  ## Format rust code
	@echo "Formatting rust code..."
	cd $(RUST_SRC_DIRECTORY) && cargo fmt

.PHONY: format-python
format-python: build-python  ## Format python code
	@echo "Formatting python code..."
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m isort .
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m ruff format
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m black .


.PHONY: lint
lint: lint-rust lint-python  ## Lint all code

.PHONY: lint-rust
lint-rust:  ## Lint rust code
	@echo "Linting rust code..."
	cd $(RUST_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(RUST_SRC_DIRECTORY) && cargo fmt --all -- --check

.PHONY: lint-python
lint-python:  ## Lint python code
	@echo "Linting python code..."
	cd $(PYTHON_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt --all -- --check
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m ruff check
	#cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m mypy .  # TODO bring back when return type annotations are added
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m isort --check-only .
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m black --check .


.PHONY: bench-rust
bench-rust:  ## Run rust benchmarks
	@echo "Running rust benchmarks..."
	cd $(RUST_SRC_DIRECTORY) && cargo bench

.PHONY: bench-python
bench-python: build-python  ## Run python benchmarks
	@echo "Running python benchmarks..."
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/


# for bench in "SequentialAugmenter/default" "SelectorAugmenter/default" "ChanceAugmenter/default" "RandomWordsAugmenter/swap" "RandomWordsAugmenter/delete" "RandomCharsAugmenter/swap" "RandomCharsAugmenter/delete" ; do \
.PHONY: profile-rust
profile-rust:  ## Produce flamegraph for rust benchmarks
	@echo "Running rust benchmarks in profile mode..."
	cd $(RUST_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow ; do \
		cd $(RUST_SRC_DIRECTORY) ; \
		cargo flamegraph --root --bench $$bench --profile $(BUILD_PROFILE) --output flamegraph-rust-bench-$$bench.svg --notes $$bench -- --bench ; \
	done

.PHONY: profile-python
profile-python: build-python  ## Produce flamegraph for python benchmarks
	@echo "Running python benchmarks in profile mode..."
	cd $(PYTHON_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow; do \
		cd $(PYTHON_SRC_DIRECTORY) ; \
		flamegraph --root --output flamegraph-python-bench-$$bench.svg --notes $$bench -- $(PYTHON_INTERPRETER) -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench-$$bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/bench_$$bench.py ; \
	done


.PHONY: compare-python
compare-python: build-python  ## Compare python bindings against other libraries
	@echo "Comparing python bindings against other libraries..."
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) benchmarks/compare_text.py --repeat $(PYTHON_COMPARE_REPETITIONS)


.PHONY: clean
clean:  ## Clean all targets
	@echo "Cleaning..."
	cd $(RUST_SRC_DIRECTORY) && cargo clean
	cd $(PYTHON_SRC_DIRECTORY) && cargo clean && rm -rf dist
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip uninstall -y fast-aug
