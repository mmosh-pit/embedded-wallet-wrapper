FROM rust:1.81 AS builder

ARG APP_NAME=kinship
WORKDIR /usr/src/app

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libclang-dev

RUN cargo init --bin .
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:stable-slim AS runtime
ARG APP_NAME=kinship

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
 && rm -rf /var/lib/apt/lists/*

RUN groupadd --system --gid 1001 appgroup && \
    useradd --system --uid 1001 --gid appgroup appuser

COPY --from=builder /usr/src/app/target/release/${APP_NAME} /app/${APP_NAME}

RUN chmod -R 777 ./

USER appuser

EXPOSE 3001

ENTRYPOINT ["/app/kinship"]
