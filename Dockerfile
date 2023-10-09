FROM rust:1.73.0-slim-bookworm

WORKDIR /usr/src/renderer

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY resources ./resources

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/ms_renderer"]