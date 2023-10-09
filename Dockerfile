FROM rust:1.73.0-slim-bookworm

# 2. Copy the files in your machine to the Docker image
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY resources ./resources

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/ms_renderer"]