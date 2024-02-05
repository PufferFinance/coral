# Guardian commands

 - [list-keys](#guardian-list-keys)
 - [rotate-key](#guardian-rotate-key)
 - [provision-next-validator](#guardian-provision-next-validator)
 - [skip-next-validator](#guardian-skip-next-validator)
 - [sign-voluntary-exit](#guardian-sign-voluntary-exit)


## `guardian list-keys`
List keys associated with this guardian

```
coral-cli guardian list-keys
```

- `--enclave-url [enclave-url]`
  - The URL to the guardian's enclave.
  - This argument is *optional* if you have it set
inside `cli.toml`


## `guardian rotate-key`
Register as a guardian

```
coral-cli guardian rotate-key \
    --private-key ./guardian_priv_key.txt
```

 - `--private-key [private-key-path]`
   - File path to a plain-text file containing the guardian's private key
   - If no private key is provided, a hardware wallet will be used (Still Work-In-Progress)

 - `--network [network]`
   - The network configuration we want to use from `cli.toml`
   - Configurable inside `cli.toml`

 - `--enclave-url [enclave-url]`
   - The URL to the guardian's enclave.
   - Configurable inside `cli.toml`

 - `--rpc-url [rpc-url]`
   - URL to the guardian's execution node.
   - Configurable inside `cli.toml`

 - `--guardian-module-address [guardian-module-address]`
   - Address of Guardian Module smart contract
   - Configurable inside `cli.toml`

## `guardian provision-next-validator`
Provision the next validator.
Made to be used with a scheduling service like cron to automate
the task of provisioning the next validator.

```
coral-cli guardian provision-next-validator \
    --private-key ./guardian_priv_key.txt \
    --public-key ./guardian_pub_key.txt
```

 - `--private-key [private-key-path]`
   - File path to a plain-text file containing the guardian's private key
   - If no private key is provided, a hardware wallet will be used (Still Work-In-Progress)

 - `--public-key [public-key-path]`
   - File path to a plain-text file containing the guardian's enclave public key

 - `--network [network]`
   - The network configuration we want to use from `cli.toml`
   - Configurable inside `cli.toml`

 - `--enclave-url [enclave-url]`
   - The URL to the guardian's enclave.
   - Configurable inside `cli.toml`

 - `--rpc-url [rpc-url]`
   - URL to the guardian's execution node.
   - Configurable inside `cli.toml`

 - `--guardian-module-address [guardian-module-address]`
   - Address of Guardian Module smart contract
   - Configurable inside `cli.toml`

 - `--puffer-protocol-address [puffer-protocol-address]`
   - Address of Puffer Protocol smart contract
   - Configurable inside `cli.toml`

## `guardian skip-next-validator`
Skip the next validator being provisioned in the queue.
This is usually used when the next validator provided invalid data.

```
coral-cli guardian skip-next-validator \
    --private-key ./guardian_priv_key.txt
```

 - `--private-key [private-key-path]`
   - File path to a plain-text file containing the guardian's private key
   - If no private key is provided, a hardware wallet will be used (Still Work-In-Progress)

 - `--network [network]`
   - The network configuration we want to use from `cli.toml`
   - Configurable inside `cli.toml`
  
 - `--enclave-url [enclave-url]`
   - The URL to the guardian's enclave.
   - Configurable inside `cli.toml`

 - `--rpc-url [rpc-url]`
   - URL to the guardian's execution node.
   - Configurable inside `cli.toml`

 - `--puffer-protocol-address [puffer-protocol-address]`
   - Address of Puffer Protocol smart contract
   - Configurable inside `cli.toml`


## `guardian sign-voluntary-exit`
Signs voluntary exit message for a misbehaving validator
in order to remove them from the validator pool

```
coral-cli guardian sign-voluntary-exit \
    --public-key guardian_pub_key.txt \
    --validator-address 0xb8f3e37fd07915e9ae13ff377dce5f215d20fd77b16d608e250aa338bbd27825c52c3659b210fb34386797451dbe9c67
```

 - `--public-key [public-key-path]`
   - File path to a plain-text file containing the guardian's enclave public key

 - `--validator-address [validator-address]`
   - Address of the validator to sign voluntary exit message for


## `guardian post-proof-of-full-withdrawals`
Post proof of full withdrawals on-chain.
Also produces a JSON file containing statistical data

```
coral-cli guardian post-proof-of-full-withdrawals \
  --private-key private-key.txt \
  --module-name 0x4e4f5f52455354414b494e47 \
  --output "puffer-reserves-$(date +'%Y%m%d_%H%M%S').json"
```

 - `--private-key [private-key-path]` *Required*
   - File path to a plain-text file containing the guardian's private key

 - `--module-name [module-name]` *Required*
   - The name of the module the validator would like to register with

 - `--output [output-path]` *Required*
   - File path to write stats to


## `guardian post-proof-of-reserves`
Post proof of reserves on-chain.
Also produces a JSON file containing all the merkle proofs and other
statistical data.

```
coral-cli guardian post-proof-of-reserves \
  --private-key private-key.txt \
  --module-name 0x4e4f5f52455354414b494e47 \
  --output "puffer-reserves-$(date +'%Y%m%d_%H%M%S').json"
```

 - `--private-key [private-key-path]` *Required*
   - File path to a plain-text file containing the guardian's private key

 - `--module-name [module-name]` *Required*
   - The name of the module the validator would like to register with

 - `--output [output-path]` *Required*
   - File path to write merkle proof to


## `guardian post-proof-of-rewards`
Post proof of rewards on-chain.
Also produces a JSON file containing all the merkle proofs and other
statistical data.

```
coral-cli guardian post-proof-of-rewards \
  --private-key private-key.txt \
  --module-name 0x4e4f5f52455354414b494e47 \
  --output "$(date +'%Y%m%d-%H%M')-reward-proof.json"
```

 - `--private-key [private-key-path]` *Required*
   - File path to a plain-text file containing the guardian's private key

 - `--module-name [module-name]` *Required*
   - The name of the module the validator would like to register with

 - `--output [output-path]` *Required*
   - File path to write merkle proof to