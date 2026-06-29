# =============================================================================
# Build stage for frontend
# =============================================================================
FROM --platform=$BUILDPLATFORM node:20-alpine AS frontend-builder

WORKDIR /build

# Install pnpm
RUN corepack enable

# Copy frontend package files
COPY web/package.json web/pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy frontend source
COPY web/ ./

# Build frontend with version injected
ARG APP_VERSION=dev
RUN NEXT_PUBLIC_APP_VERSION="${APP_VERSION}" pnpm build

# =============================================================================
# Build stage for Rust binary
# =============================================================================
FROM --platform=$BUILDPLATFORM rust:1-alpine AS rust-builder

WORKDIR /build

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY crates/octopus-core/Cargo.toml crates/octopus-core/
COPY crates/octopus-db/Cargo.toml crates/octopus-db/
COPY crates/octopus-relay/Cargo.toml crates/octopus-relay/
COPY crates/octopus-transformer/Cargo.toml crates/octopus-transformer/
COPY crates/octopus-site/Cargo.toml crates/octopus-site/
COPY crates/octopus-hub/Cargo.toml crates/octopus-hub/
COPY crates/octopus-server/Cargo.toml crates/octopus-server/
COPY crates/octopus-task/Cargo.toml crates/octopus-task/
COPY crates/octopus-cli/Cargo.toml crates/octopus-cli/

# Create dummy lib.rs files to build dependencies
RUN mkdir -p crates/octopus-core/src && echo "pub fn dummy() {}" > crates/octopus-core/src/lib.rs && \
    mkdir -p crates/octopus-db/src && echo "pub fn dummy() {}" > crates/octopus-db/src/lib.rs && \
    mkdir -p crates/octopus-relay/src && echo "pub fn dummy() {}" > crates/octopus-relay/src/lib.rs && \
    mkdir -p crates/octopus-transformer/src && echo "pub fn dummy() {}" > crates/octopus-transformer/src/lib.rs && \
    mkdir -p crates/octopus-site/src && echo "pub fn dummy() {}" > crates/octopus-site/src/lib.rs && \
    mkdir -p crates/octopus-hub/src && echo "pub fn dummy() {}" > crates/octopus-hub/src/lib.rs && \
    mkdir -p crates/octopus-server/src && echo "pub fn dummy() {}" > crates/octopus-server/src/lib.rs && \
    mkdir -p crates/octopus-task/src && echo "pub fn dummy() {}" > crates/octopus-task/src/lib.rs && \
    mkdir -p crates/octopus-cli/src && echo "fn main() {}" > crates/octopus-cli/src/main.rs

RUN cargo build --release 2>/dev/null || true

# Copy actual source code
COPY crates/ crates/

# Copy frontend build output to static directory
COPY --from=frontend-builder /build/out ./static/out

# Build arguments for version info
ARG APP_VERSION=dev
ARG GIT_COMMIT=unknown
ARG BUILD_TIME=unknown

# Build the binary
RUN cargo build --release && \
    cp target/release/octopus /usr/local/bin/octopus

# =============================================================================
# Runtime stage
# =============================================================================
FROM alpine:3.21

ARG APP_VERSION=dev
ARG GIT_COMMIT=unknown
ARG BUILD_TIME=unknown

LABEL org.opencontainers.image.version="${APP_VERSION}" \
      org.opencontainers.image.revision="${GIT_COMMIT}" \
      org.opencontainers.image.created="${BUILD_TIME}" \
      org.opencontainers.image.title="Octopus" \
      org.opencontainers.image.description="LLM API Aggregation & Load Balancing Service"

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Set default timezone
ENV TZ=Asia/Shanghai

# Create non-root user
RUN addgroup -g 1000 octopus && \
    adduser -u 1000 -G octopus -s /bin/sh -D octopus

WORKDIR /app

# Copy binary from builder
COPY --from=rust-builder /usr/local/bin/octopus .

# Copy frontend build output (server serves it from static/out at runtime)
COPY --from=rust-builder /build/static/out ./static/out

# Create data directory
RUN mkdir -p /app/data && chown -R octopus:octopus /app

# Switch to non-root user
USER octopus

# Expose port
EXPOSE 8080

# Set default data directory
ENV OCTOPUS_DATA_DIR=/app/data

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/api/v1/bootstrap/status || exit 1

# Run the binary
ENTRYPOINT ["./octopus"]
CMD ["start"]