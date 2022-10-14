#!/bin/bash
echo "cargo test"
cargo test
echo "cargo tarpaulin"
cargo tarpaulin --ignore-tests
echo "cargo clippy"
cargo clippy -- -D warnings
echo "cargo fmt"
cargo fmt -- --check
echo "cargo audit"
cargo audit
