FROM rust:1.73.0-slim-bookworm as builder

WORKDIR /usr/src/linkers

COPY ./ ./

RUN cargo build --release && cp ./target/release/ms_renderer ms_renderer && rm -rf target && rm -rf src

CMD ["./ms_renderer"]