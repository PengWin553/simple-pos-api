FROM rust:bullseye as builder

WORKDIR /app

RUN mkdir src && echo 'fn main() {}' > src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --target-dir=target --release && rm -f src/main.rs

COPY . .
ENV SQLX_OFFLINE false
RUN cargo install --target-dir=target --bin=pos-api --path .

FROM debian:bullseye-slim
ENV RUST_BACKTRACE=full
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*  \
    && rm -rf /var/cache/apt/archives/*

COPY --from=builder /usr/local/cargo/bin/pos-api /usr/local/bin/pos-api
EXPOSE 3000
CMD ["pos-api"]