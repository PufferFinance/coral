# Coral

Middleware used to talk between Enclave and Puffer smart contracts.

## Dependencies
 - [Forge](https://github.com/foundry-rs/foundry)
 - [Rust](https://www.rust-lang.org/)
 - [Secure Signer](https://github.com/PufferFinance/secure-signer)
 - [Rave](https://github.com/PufferFinance/rave)
 - [Puffer Pool](https://github.com/PufferFinance/PufferPool)

## Building

### Filesystem structure
The following instructions assume the following file structure
```
|-- coral
|   |-- coral-cli
|   |-- coral-lib
|   |-- coral-server
|   |-- lib
|-- PufferPool
|   |-- out
|       |-- bindings
|-- rave
|   |-- out
|       |-- bindings
|-- secure-signer
```

### Create Rust bindings
```
# inside PufferPool/
forge bind --crate-name puffer-pool-contracts

# inside rave/
forge bind --crate-name rave-contracts
```

### Symlink dependencies

inside coral directory create the following symlinks
```
ln -s ../PufferPool/out/bindings/ lib/PufferPool
ln -s ../rave/out/bindings/ lib/rave
ln -s ../secure-signer lib/secure-signer
```

Directory structure should look like the following after
```
|-- coral-cli
|-- coral-lib
|-- coral-server
|-- lib
|   |-- PufferPool
|   |   |-- out
|   |       |-- bindings
|   |-- rave
|   |   |-- out
|   |       |-- bindings
|   |-- secure-signer
```

### Building Coral

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

### HTTP Server (dev mode)
```
cargo run --bin coral-server
```

## Installing
```
cargo install -f --path=coral-cli
```

```
cargo install -f --path=coral-server
```

## Build binaries via Docker
```
docker buildx build -f docker/Dockerfile.builder --output type=local,dest=output .
```