# E2E test image for Rust by Building.
# Build: docker build -t rbb-e2e -f tests/e2e/Dockerfile .
# Run:   bash tests/e2e/run.sh

FROM rust:1-bookworm AS builder

WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY vendor ./vendor
COPY tools ./tools
COPY lessons ./lessons
COPY capstones ./capstones
COPY book ./book
COPY docs ./docs

# Build everything offline from the checked-in vendor tree. `--frozen`
# fails if Cargo.lock doesn't match, or if any dep isn't vendored.
# Pre-building test binaries means `rbb test` inside a fresh container
# never needs to compile (or fetch) anything.
RUN cargo build --frozen --release -p rbb -p rbb-admin
RUN cargo test  --frozen --workspace --no-run

FROM rust:1-bookworm AS runtime

# Keep the toolchain the builder used, so `rbb test` can recompile lessons.
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        git \
    && rm -rf /var/lib/apt/lists/*

# Pin a default toolchain so alice (who inherits RUSTUP_HOME via PATH
# but doesn't have her own settings) can invoke cargo.
RUN rustup default stable

# Ship the course repo + the built CLIs.
WORKDIR /opt/rbb
COPY --from=builder /src /opt/rbb
COPY --from=builder /src/target/release/rbb        /usr/local/bin/rbb
COPY --from=builder /src/target/release/rbb-admin  /usr/local/bin/rbb-admin

# Fixture student user. Home has its own checkout so admin scaffolding
# can modify /opt/rbb without polluting what the student sees.
RUN useradd -m -s /bin/bash alice \
    && cp -r /opt/rbb /home/alice/rust-by-building \
    && chown -R alice:alice /home/alice

# Pre-warm the cargo cache for both roots so tests don't wait on
# compilation for every scenario. Uses the same target/ the builder
# populated.
COPY --from=builder /src/target /opt/rbb/target
COPY --from=builder /src/target /home/alice/rust-by-building/target
RUN chown -R alice:alice /home/alice/rust-by-building/target

# Scenarios are mounted or copied in; entrypoint does nothing by default
# so scenarios drive the container fully.
COPY tests/e2e/scenarios /e2e/scenarios
COPY tests/e2e/lib.sh    /e2e/lib.sh

WORKDIR /home/alice/rust-by-building
