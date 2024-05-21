# Coral

Middleware used to talk between Enclave and Puffer smart contracts.

## Dependencies
 - [Rust](https://www.rust-lang.org/)
 - [Secure Signer](https://github.com/PufferFinance/secure-signer)

## Building

- Build with docker: [docker/README.md](docker/README.md)


### Building Coral from source

Debug mode
```
cargo build
```

release mode
```
cargo build --release
```

## Running

### ClI interface (dev mode)
```
cargo run --bin coral-cli
```

## Installing coral-cli

### Inside this repository
```
cargo install -f --path=coral-cli
```

### Remote installation without local repository cloning

```
cargo install -f --git https://github.com/PufferFinance/coral.git
```