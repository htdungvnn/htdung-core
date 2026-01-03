FROM rust:1.92 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/auth-service .
EXPOSE 50051
CMD ["./auth-service"]