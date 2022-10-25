FROM rust:1.64

WORKDIR /
COPY . .
ENV SQLX_OFFLINE true

RUN cargo build --release
ENV RUST_BACKTRACE 1
CMD ["./target/release/sim-backend"]
EXPOSE 8080