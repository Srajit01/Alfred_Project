FROM rust:1.75-alpine AS builder

# Install dependencies
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src

# Build application
RUN touch src/main.rs
RUN cargo build --release

FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/polygon-arbitrage-bot /usr/local/bin/

# Copy configuration template
COPY config.toml ./config.toml.example

# Create data directory for SQLite
RUN mkdir -p /app/data

EXPOSE 8080

CMD ["polygon-arbitrage-bot", "--config", "config.toml"]