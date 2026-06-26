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
#RUN addgroup --system app && adduser --system --ingroup app app
WORKDIR /app
COPY --from=builder /app/target/release/hangar-volume-test .
EXPOSE 8080
#USER app
CMD ["./hangar-volume-test"]
