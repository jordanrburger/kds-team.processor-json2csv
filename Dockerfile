FROM rust:1.74-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/kds-team.processor-json2csv /usr/local/bin/

ENV RUST_LOG=info

CMD ["kds-team.processor-json2csv"]
