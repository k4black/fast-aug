.DEFAULT_GOAL := help

BUILD_PROFILE ?= release

RUST_SRC_DIRECTORY = fast_aug
PYTHON_SRC_DIRECTORY = bindings/python
PYTHON_VENV_INTERPRETER = .venv/bin/python


.PHONY: all
all: help


.PHONY: help
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Arguments:"
	@echo "  BUILD_PROFILE  - release/dev"
	@echo ""
	@echo "Targets:"
	@echo "  help           - Show this help"
	@echo "  build          - Build all targets"
	@echo "  build-rust     - Build rust library"
	@echo "  build-python   - Build python library (and install)"
	@echo "  test           - Run all tests"
	@echo "  test-rust      - Run rust tests"
	@echo "  test-python    - Run python tests"
	@echo "  format         - Format all code"
	@echo "  format-rust    - Format rust code"
	@echo "  format-python  - Format python code"
	@echo "  lint           - Lint all code"
	@echo "  lint-rust      - Lint rust code"
	@echo "  lint-python    - Lint python code"
	@echo "  bench-rust     - Run rust benchmarks"
	@echo "  bench-python   - Run python benchmarks"
	@echo "  profile-rust   - Produce flamegraph for rust benchmarks"
	@echo "  profile-python - Produce flamegraph for python benchmarks"
	@echo "  clean          - Clean all targets"


.PHONY: build
build: build-rust build-python

.PHONY: build-rust
build-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo build --timings --profile $(BUILD_PROFILE)

.PHONY: build-python
build-python:
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m pip wheel . --no-deps -w dist --config-settings=build-args='--profile $(BUILD_PROFILE)' && .venv/bin/python -m pip install dist/*.whl

.PHONY: build-python-dev
build-python-dev:
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m pip install -v -e .\[test\] --config-settings=build-args='--profile $(BUILD_PROFILE)'


.PHONY: test
test: test-rust test-python

.PHONY: test-rust
test-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)

.PHONY: test-python
test-python: build-python-dev
	cd $(PYTHON_SRC_DIRECTORY) && cargo test --profile $(BUILD_PROFILE)
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m pip install maturin
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python generate_stubs.py --check
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m pytest tests/


.PHONY: format
format: format-rust format-python

.PHONY: format-rust
format-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo fmt

.PHONY: format-python
format-python:
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m isort .
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m black .


.PHONY: lint
lint: lint-rust lint-python

.PHONY: lint-rust
lint-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(RUST_SRC_DIRECTORY) && cargo fmt --all -- --check

.PHONY: lint-python
lint-python:
	cd $(PYTHON_SRC_DIRECTORY) && cargo clippy --all-targets --all-features -- -D warnings
	cd $(PYTHON_SRC_DIRECTORY) && cargo fmt --all -- --check
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m ruff check
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m mypy .
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m isort . --check-only
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m black . --check


.PHONY: bench-rust
bench-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo bench

.PHONY: bench-python
bench-python: build-python-dev
	cd $(PYTHON_SRC_DIRECTORY) && .venv/bin/python -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/


# for bench in "SequentialAugmenter/default" "SelectorAugmenter/default" "ChanceAugmenter/default" "RandomWordsAugmenter/swap" "RandomWordsAugmenter/delete" "RandomCharsAugmenter/swap" "RandomCharsAugmenter/delete" ; do \
.PHONY: profile-rust
profile-rust:
	cd $(RUST_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow ; do \
		cd $(RUST_SRC_DIRECTORY) ; \
		cargo flamegraph --root --bench $$bench --profile $(BUILD_PROFILE) --output flamegraph-rust-bench-$$bench.svg --notes $$bench -- --bench ; \
	done

.PHONY: profile-python
profile-python: build-python-dev
	cd $(PYTHON_SRC_DIRECTORY) && cargo install flamegraph
	for bench in text flow; do \
		cd $(PYTHON_SRC_DIRECTORY) ; \
		flamegraph --root --output flamegraph-python-bench-$$bench.svg --notes $$bench -- .venv/bin/python -m pytest -k bench_ --benchmark-only --benchmark-histogram=python-bench-$$bench --benchmark-name=long --benchmark-columns='min, mean, max, stddev, outliers, rounds, iterations' benchmarks/bench_$$bench.py ; \
	done


.PHONY: clean
clean:
	cd $(RUST_SRC_DIRECTORY) && cargo clean
	cd $(PYTHON_SRC_DIRECTORY) && cargo clean && rm -rf dist
