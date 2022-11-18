FROM rust:1.64

WORKDIR /
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./sqlx-data.json .
RUN mkdir ./src && echo 'pub fn main() { }' > ./src/main.rs
ENV SQLX_OFFLINE true
RUN cargo build --release
RUN rm -rf ./src
COPY . .
RUN cargo build --release
ENV RUST_BACKTRACE 1
CMD ["./target/release/sim-backend"]
EXPOSE 8080