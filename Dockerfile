FROM rust:1.88.0 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --locked --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends openssl ca-certificates && \
  apt-get autoremove -y --purge && \
  apt-get clean -y && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/short-link short-link
COPY config config

ENV SLINK_SERVER_HOST=0.0.0.0
ENTRYPOINT [ "./short-link" ]
