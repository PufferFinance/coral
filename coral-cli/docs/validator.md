# Validator commands

 - [list-keys](#validator-list-keys)
 - [register](#validator-register)
 - [sign-voluntary-exit](#validator-sign-voluntary-exit)
 - [claim-rewards](#claim-rewards)

## `validator list-keys`
List keys associated with this node

```
coral-cli validator list-keys
```

 - `--disable-enclave`
   - Disable enclave


## `validator register`
Registers validator into the Puffer Pool

```
coral-cli validator register \
    --module-name 0x4e4f5f52455354414b494e47 \
    --private-key ./validator_priv_key.txt \
    --number-of-months 12
```

 - `--module-name [module-name]` *Required*
   - The name of the module the validator would like to register with

 - `--number-of-months [number-of-months]` *Required*
   - How many months the validator intends to validate
   - This will determine how much smoothing commitment will need to be provided upfront by the validator

 - `--private-key [private-key-path]`
   - File path to a plain-text file containing the validator's private key
   - If no private key is provided, a hardware wallet will be used (Still Work-In-Progress)

 - `--disable-enclave`
   - Disable enclave

 - `--enclave-url [enclave-url]`
   - The URL to the validator's enclave.
   - Configurable inside `cli.toml`

 - `--rpc-url [rpc-url]`
   - URL to the validator's execution node.
   - Configurable inside `cli.toml`

 - `--guardian-module-address [guardian-module-address]`
   - Address of Guardian Module smart contract
   - Configurable inside `cli.toml`

 - `--puffer-protocol-address [puffer-protocol-address]`
   - Address of Puffer Protocol smart contract
   - Configurable inside `cli.toml`


## `validator sign-voluntary-exit`
Signs voluntary exit message and stop validating

```
coral-cli validator sign-voluntary-exit \
    --validator-address 0xb8f3e37fd07915e9ae13ff377dce5f215d20fd77b16d608e250aa338bbd27825c52c3659b210fb34386797451dbe9c67
```

 - `--disable-enclave`
   - Disable enclave

 - `--validator-address [validator-address]`
   - Address of the validator to sign voluntary exit message for

## `validator claim-rewards`
Allow to claim rewards for a node with merkled-proofs rewards data in a json file

```
coral-cli validator claim-rewards \
  --module-address "0x49CE199bbA75926ab5C6fc16fEDD11d418cB2EDf" \  
  --validator-pubkey "0x87e7448b5e1d4e076c7e1ff2463d3c582358ba0b8a0482e26cc7a6da5c1d94a7d5e53aad6bca39e1bb651b9d60d4da79" \  
  --rewards-path "./coral-cli/tests/data/proofs_rewards.json" \
  --private-key ./private_key.txt
```

 - `--module-address [module-address]` *Required*
   - Module address

 - `--validator-pubkey [validator-pubkey]` *Required*
   - Validator address
 
 - `--rewards-path [rewards-path]` *Required*
   - JSON file path where the rewards proofs are locally stored
   - It is possible to use multiple files by using the tag -p or --rewards-path before every path
  
 - `--private-key [private-key-path]` *Required*
   - File path to a plain-text file containing the validator's private key