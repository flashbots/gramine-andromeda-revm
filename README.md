# Andromeda MEVM in Gramine

This is a gramine environment for running Andromeda REVM (github.com/flashbots/revm-andromeda) in a TEE.

The example file so far just outputs a text execution, demoing the `Suave.localRandom` keyword.


## Run locally

Relies on gramine features, specifically `/dev/attestation/quote` and `/dev/urandom/`.
This only works right now because `/dev/urandom` works anyway.

TODO: mock out `/dev/attestation/quote` or provide alternative

```shell
cargo build
cargo run
```

## Replicate the MRENCLAVE build using Docker (no SGX Required)

```shell
docker build . --tag revm
docker run -t revm
```

## How to replicate the execution on an SGX-enabled environment (still using Docker)

```shell
docker run -it --device /dev/sgx_enclave \
       -v /var/run/aesmd/aesm.socket:/var/run/aesmd/aesm.socket \
       -v ./data:/workdir/data \
       revm "gramine-sgx ./sgx-revm"
```
