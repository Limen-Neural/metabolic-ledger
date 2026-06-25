# Dockerfile for metabolic-ledger (Rust lib crate)
# Used for Docker build verification in CI and local docker cli tests.
# Supports optional sentry feature.

FROM rust:1.88-bookworm

WORKDIR /usr/src/metabolic-ledger

# System deps for native crates (openssl, sentry etc.)
RUN apt-get update && apt-get install -y --no-install-recommends \
    "pkg-config=1.8.1*" \
    "libssl-dev=3.0.*" \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock* ./ 2>/dev/null || true
# For lib, we can cargo fetch or just copy src
COPY src ./src

# Build release with features (as in --all-features in CI)
ARG FEATURES=""
RUN if [ -n "${FEATURES}" ]; then cargo build --release --features "${FEATURES}"; else cargo build --release; fi

# For library crate with no binary, single-stage build is cleaner and more efficient for CI verification (avoids bloating final image with full target/release artifacts like .rmeta/.o as noted in reviews). Build success verifies the lib + features.
# Run as non-root for security.

# Run as non-root user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

# Since no [[bin]], the "run" is verification that build succeeded.
# Users of the lib would cargo add the crate from git/crates.
CMD ["echo", "metabolic-ledger lib built successfully"]
