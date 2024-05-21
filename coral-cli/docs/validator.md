# Validator commands

 - [list-keys](#validator-list-keys)
 - [keygen](#validator-keygen)
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

## `validator keygen`
Generates BLS keyshares to be used for registering a new validator

### with enclave
```
coral-cli validator keygen \
  --guardian-threshold 1 \
  --fork-version 0x01017000 \
  --module-name 0x4e4f5f52455354414b494e470000000000000000000000000000000000000000 \
  --withdrawal-credentials 0x01000000000000000000000049ce199bba75926ab5c6fc16fedd11d418cb2edf \
  --guardian-pubkeys 0x040783e639f5675cd12c86bab61678a2c4be44846b36df9a9648970ea803e92fd8dd25c51660b64f61d20fc04c77c34145410496fd923309a5c143b9c5eadd19e7 \
  --output-file registration_001.json \
  --enclave-url http://localhost:9001
```


### without enclave
```
coral-cli validator keygen \
  --guardian-threshold 1 \
  --fork-version 0x01017000 \
  --module-name 0x4e4f5f52455354414b494e470000000000000000000000000000000000000000 \
  --withdrawal-credentials 0x01000000000000000000000049ce199bba75926ab5c6fc16fedd11d418cb2edf \
  --guardian-pubkeys 0x040783e639f5675cd12c86bab61678a2c4be44846b36df9a9648970ea803e92fd8dd25c51660b64f61d20fc04c77c34145410496fd923309a5c143b9c5eadd19e7 \
  --output-file registration_001.json \
  --password-file passwd.txt
```

## `validator sign-voluntary-exit`
Generate signature needed to broadcast a voluntary exit message.

To be used with a beacon client or with broadcast Beaconcha.in's tool: 
For holesky: https://holesky.beaconcha.in/tools/broadcast
For mainnet: https://beaconcha.in/tools/broadcast

```
coral-cli validator sign-voluntary-exit \
  --bls-public-key 0x97cbe16970f7045cf4bf4e9bc6d3a2ae8edaba9a44308610ba4289b2da0ddb19cb4e190b455628f7e1ce8729f8d04f30 \
  --beacon-index 1695171 \
  --enclave-url http://localhost:9002 \
  --fork-current-version 0x04017000 \
  --fork-previous-version 0x03017000 \
  --epoch 256 \
  --genesis-validators-root 0x9143aa7c615a7f7115e2b6aac319c03529df8242ae705fba9df39b79c59fa8b1 \
  --output-file sign_vem_001.json
```