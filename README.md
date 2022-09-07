# scsys/chaos

[![Code Analysis](https://github.com/Scattered-Systems/chaos/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Scattered-Systems/chaos/actions/workflows/rust-clippy.yml)
[![Docker](https://github.com/Scattered-Systems/chaos/actions/workflows/docker.yml/badge.svg)](https://github.com/Scattered-Systems/chaos/actions/workflows/docker.yml)
[![Rust](https://github.com/Scattered-Systems/chaos/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/chaos/actions/workflows/rust.yml)

Welcome to Chaos

## Getting Started

### Building from the source


    git clone https://github.com/scattered-systems/chaos
    cargo build --release --workspace

#### _Testing_

    cargo test --all-features --release --verbose

#### _Run_

or

    cargo run -p chaos --bin --release

