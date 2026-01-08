# Validator commands

 - [generate-bls-key](#validator-generate-bls-key)
 - [keygen](#validator-keygen)
 - [list-keys](#validator-list-keys)
 - [sign-voluntary-exit](#validator-sign-voluntary-exit)

## `validator generate-bls-key`
Generate a new BLS key pair and save the keystore to a file.

This is a standalone command for generating BLS keys without creating deposit data.

```
coral-cli validator generate-bls-key \
  --password-file passwd.txt \
  --output-dir ./my-keys
```

### Arguments

- `--password-file` [file-path] (required)

  Path to a file containing the password for encrypting the keystore. Password must be at least 8 characters.

- `--output-dir` [directory-path] (optional)

  Directory where the keystore file will be saved. Defaults to `./etc/keys/bls_keys`.

### Output

The command outputs:
- The generated BLS public key (hex encoded)
- The path to the encrypted keystore file (named `<pubkey>.json`)

## `validator keygen`
Generates a BLS key and deposit data for registering a new validator.

This command generates a BLS key pair, creates the deposit signature for 32 ETH, and outputs all data needed for validator registration.

```
coral-cli validator keygen \
  --module-name 0x4e4f5f52455354414b494e470000000000000000000000000000000000000000 \
  --withdrawal-credentials 0x01000000000000000000000049ce199bba75926ab5c6fc16fedd11d418cb2edf \
  --fork-version 0x01017000 \
  --password-file passwd.txt \
  --output-file registration_001.json
```

### Arguments

- `--module-name` [hex-string] (required)

  The module name as a 32-byte hex string.

- `--withdrawal-credentials` [hex-string] (required)

  The withdrawal credentials as a 32-byte hex string. Typically starts with `0x01` for ETH1 withdrawal addresses.

- `--fork-version` [hex-string] (required)

  The genesis fork version as a 4-byte hex string (e.g., `0x01017000` for Holesky).

- `--password-file` [file-path] (required)

  Path to a file containing the password for encrypting the keystore. Password must be at least 8 characters.

- `--output-file` [file-path] (required)

  Path where the registration JSON output will be saved.

### Output

The command outputs a JSON file containing:
- `version`: The coral-cli version
- `module_name`: The module name (hex encoded)
- `withdrawal_credentials`: The withdrawal credentials (hex encoded)
- `fork_version`: The fork version used for signing
- `bls_pub_key`: The generated BLS public key (hex encoded)
- `signature`: The deposit signature (hex encoded)
- `deposit_data_root`: The deposit data root hash (hex encoded)

The BLS keystore is saved to `etc/keys/bls_keys/<pubkey>.json`.

## `validator list-keys`
List BLS keys from the local keystore.

```
coral-cli validator list-keys \
  --keystore-path ./etc/keys/bls_keys
```

### Arguments

- `--keystore-path` [directory-path] (optional)

  Path to the directory containing keystore files. Defaults to `./etc/keys/bls_keys`.

## `validator sign-voluntary-exit`

**Note:** This command is currently not supported. Validator exits will be supported on-chain using [EIP-7002](https://eips.ethereum.org/EIPS/eip-7002).

## `validator verify-merkle-tree-rewards`
From an input file containing rewards data between 2 epochs, this command generates the merkle tree root and proofs and compares it from the given merkle root.

```
coral-cli validator verify-merkle-tree-rewards \
--rewards-file-path ./coral-cli/src/tests/rewards-files/rewards_280402_315550.json \
--rpc-url https://ethereum-rpc.publicnode.com
```


- `--rewards-file-path` [file-path]

Path of the rewards file
You can find this link in  `mint_and_bridge_rewards` transactions calldata.

- `--rpc-url`  [rpc-url]

RPC url for which our smart contract function mint and bridge rewards has been deployed.
It would been holesky's rpc url or mainnet rpc url. 

You can use for example the public ones : 
- [holesky](https://ethereum-holesky-rpc.publicnode.com)
- [mainnet](https://ethereum-rpc.publicnode.com)