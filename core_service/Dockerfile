FROM rust:1.64

WORKDIR /
COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release
CMD ["./target/release/core-service"]