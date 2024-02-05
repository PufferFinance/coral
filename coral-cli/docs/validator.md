# Validator commands

 - [list-keys](#validator-list-keys)
 - [register](#validator-register)

## `validator list-keys`
List keys associated with this node

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