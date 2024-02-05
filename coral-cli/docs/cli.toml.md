# cli.toml

`coral-cli` will look in the following directories for a `cli.toml` file:
 - `CORAL_CONFIG_DIR` environment variable
 - `~/.config/puffer/coral`

## All options available
```
# Validator config
[validator]
# Enclave URL
enclave_url = "http://localhost:9001"

# Where to store keys and passwords if not using enclave
keystore_path = "keystore"
password_path = "password"

# Guardian config
[guardian]
# Enclave URL
enclave_url = "http://localhost:9003"

# Coral config
[coral]
selected_network = "ephemery"


# Network configs

# Network called 'ephemery'
[network.ephemery]

# API URL to ETH execution node
rpc_url = "http://localhost:8545"

# API URL to ETH beacon node
beacon_url = "http://localhost:5051"

# Network fork information
[network.ephemery.fork]
previous_version = "0x1000101a"
current_version = "0x1000101b"
genesis_validators_root = "0x74ae0047bd8a34c96ce1f060886b7fd55f289d21e277f712338fd33a840dd7f0"
epoch = 0

# Puffer contracts for this network
[network.ephemery.puffer]
puffer_pool_address = "0xd6D06b62bB2Af6A3378d55CEeb7B0d52ee77b892"
puffer_protocol_address = "0xD3099adAadE062138c5a71836e40F68fe5954057"
guardian_module_address = "0xB1342027Ecd6A2e9Cb393e3830d3cA3A0eFdC220"
```