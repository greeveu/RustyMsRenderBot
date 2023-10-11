FROM rust:1.73.0-slim-bookworm as builder

WORKDIR /usr/src/renderer

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY resources ./resources

RUN cargo build --release

FROM scratch

WORKDIR /usr/src/renderer

COPY --from=builder /usr/src/renderer/target/release/ms_renderer ./ms_renderer
CMD ["./ms_renderer"]