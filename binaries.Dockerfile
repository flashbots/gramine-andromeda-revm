FROM ruteri/gramine-andromeda-revm:latest as builder # Temporary, will be flashbots once tested

FROM scratch as binaries

COPY --from=builder /workdir/sgx-revm.sig /
COPY --from=builder /workdir/sgx-revm.manifest /
COPY --from=builder /workdir/sgx-revm.manifest.sgx /
COPY --from=builder /workdir/target/release/gramine-sirrah /target/release/
