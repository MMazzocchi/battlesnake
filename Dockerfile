# Build stage
FROM rust AS builder

ENV CARGO_HOME /code/.cargo
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update
RUN apt-get install musl-tools -y

RUN mkdir -p /code
WORKDIR /code

COPY Cargo.lock ./
COPY Cargo.toml ./

# Install and build dependencies
RUN mkdir -p ./src
RUN echo "// Build" >> src/lib.rs

RUN cargo update
RUN cargo build --release --target x86_64-unknown-linux-musl

# Build executable
COPY src/ ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

# Execute stage
FROM alpine 
RUN apk add libc6-compat libgcc

RUN mkdir -p /battlesnake
WORKDIR /battlesnake

COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/battlesnake ./
COPY config/ ./config
COPY static/ ./static

CMD /battlesnake/battlesnake 
