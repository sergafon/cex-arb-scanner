ARG BIN=arbitrage-scanner



FROM rust:1.88-slim AS base

WORKDIR /app/backend

RUN apt-get update -qq \
 && apt-get install -y --no-install-recommends \
      libssl-dev pkg-config build-essential ca-certificates \
 && rm -rf /var/lib/apt/lists/*



FROM base AS dev

ENV CARGO_INCREMENTAL=1

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    rustup component add clippy rustfmt && \
    cargo install --locked cargo-watch

COPY ./backend/Cargo.toml ./backend/Cargo.lock ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    mkdir -p src && echo "fn main(){println!(\"stub\")}" > src/main.rs && \
    cargo fetch && cargo build && rm -rf src

COPY ./backend/. .



FROM base AS builder

COPY ./backend/Cargo.toml ./backend/Cargo.lock ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    mkdir -p src && echo "fn main(){println!(\"stub\")}" > src/main.rs && \
    cargo fetch && rm -rf src

COPY ./backend/. .

ARG BIN

RUN --mount=type=cache,target=/app/backend/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --bin "$BIN" && \
    install -Dm755 "target/release/${BIN}" "/app/backend/${BIN}"



FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -qq \
 && apt-get install -y --no-install-recommends ca-certificates curl \
 && rm -rf /var/lib/apt/lists/* \
 && useradd -u 10001 -m app

ARG BIN

COPY --from=builder /app/backend/${BIN} /usr/local/bin/app

USER app

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/app"]
