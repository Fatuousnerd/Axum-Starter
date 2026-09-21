# Stage 1: Compute a recipe file for dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1.80-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build dependencies and binary
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin axum-starter

# Stage 3: Final minimal runtime environment
FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/axum-starter /app/backend

# Run as non-root user for security
USER 1000
EXPOSE 8080
CMD ["./backend"]
