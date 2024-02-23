# Validator commands

 - [list-keys](#validator-list-keys)
 - [register](#validator-register)

## `validator list-keys`
List keys associated with this node

For validators running the secure-signer enclave:
```
coral-cli validator list-keys \
  --enclave-url http://localhost:9001 
```

For validators not running the secure-signer enclave:
```
coral-cli validator list-keys \
  --disable-enclave \
  --keystore_path ~/.puffer/coral/keystore
```

## `validator register`
Registers validator into the Puffer Pool

```
coral-cli validator register \
  --guardian-threshold 1 \
  --fork-version 0x01017000 \
  --enclave-url http://localhost:9001 \
  --withdrawal-credentials 0x01000000000000000000000049ce199bba75926ab5c6fc16fedd11d418cb2edf
  --output-file registration_001.json \
  --guardian-pubkeys 0x040783e639f5675cd12c86bab61678a2c4be44846b36df9a9648970ea803e92fd8dd25c51660b64f61d20fc04c77c34145410496fd923309a5c143b9c5eadd19e7
```

## `validator sign-voluntary-exit`
Sign voluntary exit message and stop validating

```
coral-cli validator sign-voluntary-exit \
  --bls_pukey 0x0439afaa6c9c26c342c7463d8fd116d72f18ed1980a10bd6a2707a2a498315bb17ff4352b91c18b2d34391503fe15fe48b42f95fafc96d4a758172de61dd9a91e7 \
  --enclave-url http://localhost:9001 \
  --beacon_url http://localhost:5052 \
  --fork-previous-version 0x01017000 \
  --fork-current-version 0x01017000 \
  --epoch 5
  --genesis_validators_root 0x270d43e74ce340de4bca2b1936beca0f4f5408d9e78aec4850920baf659d5b69 \
  --output-file registration_001.json \
```