FROM gramineproject/gramine:v1.5

RUN apt-get update && apt-get install -y jq build-essential libclang-dev

WORKDIR /workdir

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install 1.72.0

RUN gramine-sgx-gen-private-key

# Build just the dependencies (shorcut)
COPY Cargo.lock Cargo.toml ./
RUN mkdir src && touch src/lib.rs
RUN --mount=type=ssh  cargo build --release
RUN rm src/lib.rs

# Now add our actual source
COPY Makefile README.md sgx-revm.manifest.template ./
COPY src/main.rs ./src/
COPY src/examples_Andromeda_sol_Andromeda.bin ./src/

# Build with rust
RUN cargo build --release

# Make and sign the gramine manifest
RUN make SGX=1 RA_TYPE=dcap

CMD [ "gramine-sgx-sigstruct-view sgx-revm.sig" ]
