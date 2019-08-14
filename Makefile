SHELL := /bin/bash

.PHONY: help
help:
	@echo "Please use \`make <target>\` where <target> is one of"
	@echo "  all               to check, build, test and release this SDK"
	@echo "  check             to vet and lint the SDK"
	@echo "  update            to update git submodules"
	@echo "  generate          to generate service code"
	@echo "  build             to build the SDK"
	@echo "  test              to run test"
	@echo "  integration-test  to run integration test"

.PHONY: all
all: check build unit release

.PHONY: check
check: cargo check

.PHONY: update
update:
	git submodule update --remote
	@echo "Done"

.PHONY: generate
generate:
	@if [[ ! -f "$$(which snips)" ]]; then \
		echo "ERROR: Command \"snips\" not found."; \
	fi
	snips -f="./specs/qingstor/2016-01-06/swagger/api_v2.0.json" -t="./template" -o="./src/service"
	@echo "Done"


.PHONY: build
build:
	@echo "Build"
	cargo build --verbose
	@echo "Done"

RUNTESTCASE = _run_test_case() {                                                  \
    case="$(filter-out $@,$(MAKECMDGOALS))";                                      \
    if [ -n "$${case}" ]; then                                                    \
        RUST_BACKTRACE=full cargo test $${case} -- --nocapture --test-threads=1;  \
    else                                                                          \
        RUST_BACKTRACE=full cargo test -- --nocapture --test-threads=1;           \
    fi  \
}

.PHONY: test
test:
	@echo "Run test"
	@${RUNTESTCASE}; _run_test_case
	@echo "Done"

.PHONY: integration-test
integration-test:
	@echo "Run integration test"
	pushd "./test"; go run *.go; popd
	@echo "Done"

.PHONY: clean
clean:
	rm -rf $${PWD}/coverage
	@echo "Done"


.DEFAULT_GOAL = build
