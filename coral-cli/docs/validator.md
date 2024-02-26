# Validator commands

 - [list-keys](#validator-list-keys)
 - [register](#validator-register)
 - [sign-voluntary-exit](#validator-sign-voluntary-exit)

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
  --bls-public-key 0x94505f60bb8e48ddafb8835ec08537c48ed1bb9bc6a95fe941f37869b5eb0950c9023b7a997fe12d8aa79076561e009f \
  --beacon-index 1605300 \
  --enclave-url http://localhost:9001 \
  --fork-previous-version 0x04017000 \
  --fork-current-version 0x05017000 \
  --epoch 29696 \
  --genesis-validators-root 0x9143aa7c615a7f7115e2b6aac319c03529df8242ae705fba9df39b79c59fa8b1 \
  --output-file sign_vem_001.json
```