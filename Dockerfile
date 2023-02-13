
# Define custom function directory

# ---- Base builder ----
FROM rust:1.67 AS rust-builder

RUN apt-get update && apt-get install -y \
    g++-x86-64-linux-gnu libc6-dev-amd64-cross \
    g++-aarch64-linux-gnu libc6-dev-arm64-cross && \ 
    rm -rf /var/lib/apt/lists/*

RUN rustup target add \
    x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
RUN rustup toolchain install \
    stable-x86_64-unknown-linux-gnu stable-aarch64-unknown-linux-gnu
RUN rustup component add rustfmt

# x86_64-unknown-linux-gnu
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
    CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc \
    CXX_x86_64_unknown_linux_gnu=x86_64-linux-gnu-g++ 
    
# aarch64-unknown-linux-gnu
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ 

ENV CARGO_INCREMENTAL=0

# --- Arm64 build ----
FROM rust-builder AS build-arm64

ENV LAMBDA_TASK_ROOT="/var/task"
WORKDIR ${LAMBDA_TASK_ROOT} 

# Cache dependencies
ADD Cargo.toml .
ADD Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release --target aarch64-unknown-linux-gnu

# We need to touch our real main.rs file or else docker will use
# the cached one.
ADD src src

# build for release
RUN cargo install --target aarch64-unknown-linux-gnu --path .
RUN mv ./target/aarch64-unknown-linux-gnu/release/rust-api-template /usr/bin/rust-api-template

# ---- Lambda arm64 final build ----
FROM python:buster AS final-lambda-arm64

COPY --from=build-arm64 /usr/bin/rust-api-template/ usr/bin/rust-api-template

CMD ["rust-api-template"]
