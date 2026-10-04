FROM rust:1.82-slim-bookworm AS bangun
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=bangun /app/target/release/pawonee-backend-service /usr/local/bin/pawonee
COPY migrations /migrations
ENV RUST_LOG=info
EXPOSE 8080
CMD ["pawonee"]
