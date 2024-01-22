# Andromeda MEVM in Gramine

This is a gramine environment for running Andromeda REVM (github.com/flashbots/revm-andromeda) in a TEE.

The example file so far just outputs a test execution, demoing the `Suave.localRandom` precompile.

TODO: 
- Interface to untrusted host. What we should do is accept commands (`provideBlock`, `ethCall`) on `stdin`... providing call results on `stdout`.
   - The `provideBlock` would be for advancing the light client forward
   - `ethCall` would be for invoking an offchain confidential query. We can either require this info passed in ahead of time, or request it on demand through `stdin`/`stdout`.
- Import light client. The enclave should only advance forward on validated claims. The enclave should only execute evm in valid contexts.

## Run locally

The Andromeda `revm-andromeda` relies on gramine features for the precompiles, specifically `/dev/attestation/quote` and `/dev/urandom/`.
Running outside of an enclave, we can still simulate this. For example `/dev/urandom` works anyway. The other Andromeda precompiles, `volatile{Get/Set}` are directly managed in-memory by `revm-andromeda`. 

TODO: mock out `/dev/attestation/quote` or provide alternative

```shell
git submodule update --init # temporary until repositories are public, fetch the private dependencies
cargo build
cargo run
```

## Replicate build using Docker (no SGX Required)
To build and print the MRENCLAVE:
```shell
docker build . --tag gramine-andromeda-revm
docker run -t gramine-andromeda-revm
```

## How to replicate the execution on an SGX-enabled environment (still using Docker)

```shell
docker run -it --device /dev/sgx_enclave \
       -v /var/run/aesmd/aesm.socket:/var/run/aesmd/aesm.socket \
       gramine-andromeda-revm "gramine-sgx ./sgx-revm"
```
