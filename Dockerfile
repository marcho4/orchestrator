FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /app/target/release/orchestrator /app/orchestrator
EXPOSE 8080
CMD ["./orchestrator"]