> [!WARNING]
> This repository is a work in progress, and for now only functions as a showcase. This code *is not intended to secure any valuable information*.

# Andromeda MEVM in Gramine

This is a gramine environment for running [Andromeda REVM](github.com/flashbots/suave-andromeda-revm) in a TEE.

The TEE service (gramine-sirrah) uses stdin and stdout for passing data in and out of the REVM, which currently supports two commands:
* `advance [height]`, which advances the suave chain to requested height (or to latest if no height provided)
* `execute tx_data`, which executes the requested data. For data format see [Andromeda REVM](github.com/flashbots/suave-andromeda-revm).

The TEE service is stateless, so make sure that you have `suave-geth` running. TEE will connect to `http://localhost:8545` by default, which you can override by passing `--rpc` flag. The RPC is used for fetching chain data along with their proofs.

We also provide a simple http and tpc [server](server.py) for handling requests to and from the TEE service, for example usage see [andromeda-sirrah-contracts](github.com/flashbots/andromeda-sirrah-contracts).

## Current measurement

```
mr_signer: f0365ce7081fda379914c703fe08648db1cce3747e8c10f74ff742926399f15a
mr_enclave: cd4a0af69a811a223a92379b08a84b2e3ecdd7a199bb0656ebd1cd689df8c402
```

## Run locally

The Andromeda `revm-andromeda` relies on gramine features for the precompiles, specifically `/dev/attestation/quote` and `/dev/urandom/`.  
Running outside of an enclave, we can still simulate this. For example `/dev/urandom` works anyway. The other Andromeda precompiles, `volatile{Get/Set}` are directly managed in-memory by `suave-andromeda-revm`. 

```shell
cargo build
cargo run
```

## Replicate build using Docker (no SGX Required)
To build and print the MRENCLAVE:
```shell
docker build --tag gramine-andromeda-revm .
docker run --rm gramine-andromeda-revm
```

## Extract reproducible binaries built using docker

```shell
docker build --output=. -f=binaries.Dockerfile .
```
Alternatively, run `make docker-binaries` which does the same. Note that the binaries will be pulled from dockerhub as opposed to local image. This ensures the MRSIGNER is matching.  

The above will output sgx-revm.sig, sgx-revm.manifest, sgx-revm.manifest.sgx into the main directory, and gramine-sirrah into target/release directory. Continue as if you just ran `SGX=1 make all`. Since we are outputing the binaries, you might encounter errors if you are not using the same OS as the docker target (ubuntu 22.04).

For now we are still checking the `mr_signer`, if you want to connect to one of the predeployed Andromeda contracts without configuring your `mr_signer` you can pull binaries from `ruteri/gramine-andromeda-revm:latest`.

## How to replicate the execution on an SGX-enabled environment (still using Docker)

```shell
docker run -it --device /dev/sgx_enclave \
       -v /var/run/aesmd/aesm.socket:/var/run/aesmd/aesm.socket \
       gramine-andromeda-revm "gramine-sgx ./sgx-revm"
```

## License

The code in this project is free software under the [MIT license](LICENSE).
