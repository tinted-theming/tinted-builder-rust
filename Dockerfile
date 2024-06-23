FROM rust:latest as builder
WORKDIR /usr/src/tinted-builder-rust

# Copy required compilation source
COPY Cargo.toml Cargo.lock ./
COPY tinted-builder ./tinted-builder
COPY tinted-builder-rust ./tinted-builder-rust

# Build and test
RUN cargo build -p tinted-builder-rust --release
RUN cargo test -p tinted-builder-rust --release

# Sync since there is no reason not to include the schemes
RUN /usr/src/tinted-builder-rust/target/release/tinted-builder-rust sync

FROM rust:latest
COPY --from=builder /usr/src/tinted-builder-rust/target/release/tinted-builder-rust /usr/local/bin/tinted-builder-rust

ENTRYPOINT ["tinted-builder-rust"]
