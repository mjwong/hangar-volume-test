FROM rust:1.78-slim AS builder
WORKDIR /app
COPY Cargo.toml .
# Cache dependencies
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build --release && rm -rf src
COPY src src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN addgroup --system app && adduser --system --ingroup app app \
    mkdir -p /data && chown app:app /data
WORKDIR /app
COPY --from=builder /app/target/release/hangar-volume-test .
USER app
EXPOSE 8080
CMD ["./hangar-volume-test"]