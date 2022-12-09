# scsys/chaos

[![Code Analysis](https://github.com/scattered-systems/chaos/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/chaos/actions/workflows/clippy.yml)
[![Docker](https://github.com/scattered-systems/chaos/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/chaos/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/chaos/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/chaos/actions/workflows/rust.yml)

Welcome to Chaos

## Getting Started

### Building from the source

```bash
git clone https://github.com/scattered-systems/chaos
cargo build --release --workspace
```

#### _Testing_

```bash
cargo test --all-features --release --verbose
```

#### _Run_

```bash
cargo run -- -h
```

### Docker

Start by cloning the repository

```bash
docker pull scsys/chaos:latest
```

#### _Build a new image_

```bash
docker buildx build --tag scsys/chaos:latest .
```

### Usage

```rust

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
