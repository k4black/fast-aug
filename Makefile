.DEFAULT_GOAL := help

BUILD_PROFILE ?= release
PYTHON_INTERPRETER ?= $(CURDIR)/bindings/python/.venv/bin/python
COMPARE_REPETITIONS ?= 10

RUST_SRC_DIRECTORY = $(CURDIR)/fast_aug
PYTHON_SRC_DIRECTORY = $(CURDIR)/bindings/python


.PHONY: help
help:  ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


.PHONY: build
build: build-rust build-python  ## Build all targets

.PHONY: build-rust
build-rust:  ## Build rust library
	cd $(RUST_SRC_DIRECTORY) && cargo build --timings --profile $(BUILD_PROFILE)

.PHONY: build-python
build-python:  ## Build python library (and install)
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip wheel . --no-deps -w dist --config-settings=build-args='--profile $(BUILD_PROFILE)' && $(PYTHON_INTERPRETER) -m pip install dist/*.whl

.PHONY: build-python-dev
build-python-dev:  ## Build python library (and install in editable mode)
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install -v -e .\[test\] --config-settings=build-args='--profile $(BUILD_PROFILE)'
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) generate_stubs.py


.PHONY: generate-stubs
generate-stubs: build-python-dev  ## Generate python stubs
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) generate_stubs.py


.PHONY: test
test: test-rust test-python  ## Run all tests

.PHONY: test-rust
test-rust:  ## Run rust tests
	cd $(RUST_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)

.PHONY: test-python
test-python: build-python-dev  ## Run python tests
	cd $(PYTHON_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install maturin
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) generate_stubs.py --check
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pytest tests/


.PHONY: format
format: format-rust format-python  ## Format all code

.PHONY: format-rust
format-rust:  ## Format rust code
	cd $(RUST_SRC_DIRECTORY) && cargo fmt

.PHONY: format-python
format-python: build-python-dev  ## Format python code
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m isort .
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m black .


.PHONY: lint
lint: lint-rust lint-python  ## Lint all code

.PHONY: lint-rust
lint-rust:  ## Lint rust code
	cd $(RUST_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(RUST_SRC_DIRECTORY) && cargo fmt --all -- --check

.PHONY: lint-python
lint-python:  ## Lint python code
	cd $(PYTHON_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt --all -- --check
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m ruff check
	#cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m mypy .  # TODO bring back when return type annotations are added
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m isort --check-only .
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m black --check .


.PHONY: bench-rust
bench-rust:  ## Run rust benchmarks
	cd $(RUST_SRC_DIRECTORY) && cargo bench

.PHONY: bench-python
bench-python: build-python-dev  ## Run python benchmarks
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/


# for bench in "SequentialAugmenter/default" "SelectorAugmenter/default" "ChanceAugmenter/default" "RandomWordsAugmenter/swap" "RandomWordsAugmenter/delete" "RandomCharsAugmenter/swap" "RandomCharsAugmenter/delete" ; do \
.PHONY: profile-rust
profile-rust:  ## Produce flamegraph for rust benchmarks
	cd $(RUST_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow ; do \
		cd $(RUST_SRC_DIRECTORY) ; \
		cargo flamegraph --root --bench $$bench --profile $(BUILD_PROFILE) --output flamegraph-rust-bench-$$bench.svg --notes $$bench -- --bench ; \
	done

.PHONY: profile-python
profile-python: build-python-dev  ## Produce flamegraph for python benchmarks
	cd $(PYTHON_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow; do \
		cd $(PYTHON_SRC_DIRECTORY) ; \
		flamegraph --root --output flamegraph-python-bench-$$bench.svg --notes $$bench -- $(PYTHON_INTERPRETER) -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench-$$bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/bench_$$bench.py ; \
	done


.PHONY: compare-python
compare-python:  ## Compare python bindings against other libraries
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) -m pip install .\[compare\]
	cd $(PYTHON_SRC_DIRECTORY) && $(PYTHON_INTERPRETER) benchmarks/compare_text.py --repeat $(COMPARE_REPETITIONS)


.PHONY: clean
clean:  ## Clean all targets
	cd $(RUST_SRC_DIRECTORY) && cargo clean
	cd $(PYTHON_SRC_DIRECTORY) && cargo clean && rm -rf dist
