# Multi-stage Dockerfile for metabolic-ledger (Rust lib crate)
# Used for Docker build verification in CI and local docker cli tests.
# Supports optional sentry feature.

FROM rust:1.88-bookworm AS builder

WORKDIR /usr/src/metabolic-ledger

# System deps for native crates (openssl, sentry etc.)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./
# For lib, we can cargo fetch or just copy src
COPY src ./src

# Build release with features (as in --all-features in CI)
RUN cargo build --release --features sentry

# Final stage (minimal; since lib, mainly for build artifact presence)
FROM debian:bookworm-slim

WORKDIR /app

# Copy built artifacts (lib, rmeta etc for verification)
COPY --from=builder /usr/src/metabolic-ledger/target/release /app/release-artifacts

# Since no [[bin]], the "run" is verification that build succeeded.
# Users of the lib would cargo add the crate from git/crates.
CMD ["echo", "metabolic-ledger lib built successfully (with sentry feature)"]
