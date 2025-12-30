FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app
RUN apt update && apt install mold clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
WORKDIR /app
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin avina-api

FROM debian:trixie-slim as runtime
WORKDIR /app
RUN apt update -y \
    && apt install -y --no-install-recommends openssl ca-certificates \
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/avina-api avina-api
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./avina-api"]
