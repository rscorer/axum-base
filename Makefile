.PHONY: test test-api test-cli test-all check

# Run all tests single-threaded (default target)
test:
	cargo test -- --test-threads=1

# Run only API tests
test-api:
	cargo test --test api_tests -- --test-threads=1

# Run only CLI tests  
test-cli:
	cargo test --test cli_tests -- --test-threads=1

# Run all tests with output
test-all:
	cargo test -- --test-threads=1 --nocapture

# Quick compile check
check:
	cargo check --tests

# Clean and test
clean-test:
	cargo clean && make test
