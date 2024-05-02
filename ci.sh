#!/bin/bash

cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo tarpaulin --verbose --workspace