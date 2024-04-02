# Validator dev commands

These commands are not available in production build
of `coral-cli` and is to be used for ***testing purposes only***.

## `validator register-key`
***FOR TESTING ONLY***

Registers validator into the Puffer Pool

```
coral-cli validator register-key \
  --private-key 0x829348903124902... \
  --rpc-url http://localhost:8545 \
  --puffer-oracle-address 0x123... \
  --puffer-protocol-address 0x123... \
  --validator-ticket-address 0x123... \
  --module-name 0x123... \
  --number-of-days 30 \
  --input-file registration_001.json
```

## `validator generate-register-calldata`
***FOR TESTING ONLY***

Generate register-key calldata to be used for testing.

```
coral-cli validator generate-register-calldata \
  --rpc-url http://localhost:8545 \
  --puffer-protocol-address 0x797e7b4B55a2ae28796e4631180EdaB4Aa0CF5C3 \
  --validator-ticket-address 0xCf944F6f6Fc77faFF9600388Beaa5dF2C9a91Fed \
  --module-name 0x4e4f5f52455354414b494e470000000000000000000000000000000000000000 \
  --input-file registration_20240319_001.json
```

## `validator withdrawal-credentials`
***FOR TESTING ONLY***

Fetch withdrawal credentials for a given puffer module address

```
coral-cli validator withdrawal-credentials \
  --rpc-url http://localhost:8545 \
  --puffer-protocol-address 0x797e7b4B55a2ae28796e4631180EdaB4Aa0CF5C3 \
  --module-address 0x1234... \
  --input-file registration_20240319_001.json
```