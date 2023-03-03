FROM rust:1.64

WORKDIR /
COPY . .

ENV SQLX_OFFLINE true
ENV CARGO_NET_GIT_FETCH_WITH_CLI true

RUN cargo build --release
CMD ["./target/release/auth-service"]