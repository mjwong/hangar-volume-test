FROM rust:1.78-slim AS builder
WORKDIR /app
COPY Cargo.toml .
# Cache dependencies
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build --release && rm -rf src
COPY src src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
ARG HANGAR_BASE_PATH
ARG HANGAR_DEPLOYMENT_ID
WORKDIR /app
COPY --from=builder /app/target/release/hangar-hello-rust .
EXPOSE 8080
CMD ["./hangar-hello-rust"]
