#!/usr/bin/env bash
cargo test --workspace --quiet --color always
cargo build --release --workspace
