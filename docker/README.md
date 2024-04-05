# docker

## Build image
```
docker buildx build -f docker/Dockerfile -t coral-cli:latest .
```

## Running Image
```
docker run -it --network host --rm coral-cli:latest /app/coral-cli version
```

## Running image with volume to get output - *only for Linux* 

### Keygen (with enclave)
```
docker run -it --network host \
    -v ./output:/app/output --rm \
    coral-cli:latest /app/coral-cli validator keygen \
    --guardian-threshold 1 \
    --module-name 0x5055464645525f4d4f44554c455f300000000000000000000000000000000000 \
    --withdrawal-credentials 0x0100000000000000000000005ee9246f01e95c08ee767029c1d18765bb1779d0 \
    --guardian-pubkeys 0x049cc1fbaa3cffd3e4c1f935c47720d013938ccb822a9cbd20c5f09ab65ae8300e7986b6ce75e916d3b59599ece72134adf2972d06a76a8ba5f3747d356117c342 \
    --fork-version 0x01017000 \
    --enclave-url http://localhost:9001 \
    --output-file output/registration_docker_001.json
```

### Keygen (no enclave)
```
docker run -it --network host \
    -v ./output:/app/output --rm \
    coral-cli:latest /app/coral-cli validator keygen \
    --guardian-threshold 1 \
    --module-name 0x5055464645525f4d4f44554c455f300000000000000000000000000000000000 \
    --withdrawal-credentials 0x0100000000000000000000005ee9246f01e95c08ee767029c1d18765bb1779d0 \
    --guardian-pubkeys 0x049cc1fbaa3cffd3e4c1f935c47720d013938ccb822a9cbd20c5f09ab65ae8300e7986b6ce75e916d3b59599ece72134adf2972d06a76a8ba5f3747d356117c342 \
    --fork-version 0x01017000 \
    --password-file output/passwd.txt \
    --output-file output/registration_docker_001.json
```

## Build binaries via Docker and output onto host system
```
docker buildx build -f docker/Dockerfile --output type=local,dest=output .
```