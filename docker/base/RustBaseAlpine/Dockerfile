FROM rust:1.95.0-alpine3.23

# Use a more minimal approach with explicit layer management
RUN apk update && \
    apk add --no-cache \
        alpine-sdk \
        sqlite-dev \
        openssl-dev \
        wget \
        ca-certificates \
        nmap \
        postgresql16-client \
        libsmbclient \
        samba-dev \
        libnfs-dev \
        zlib-dev \
        git \
        curl \
        && update-ca-certificates

# Install wait utility
RUN wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.12.1/wait \
    && chmod +x /wait

# Install sccache
ARG SCCACHE_VERSION=0.14.0
RUN set -eux; \
    wget -q "https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz"; \
    tar -xzf "sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz"; \
    mv "sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache" /usr/local/bin/sccache; \
    chmod +x /usr/local/bin/sccache; \
    rm -rf "sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz" "sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl"

# Install cargo-chef
RUN cargo install cargo-chef

# Set environment variables for build optimization
ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_REDIS="redis://mksccache:6379/mksccache"
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Create appuser with proper uid/gid
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "1000" \
    "mediakraken"

# Clean up any unnecessary packages or files
RUN rm -rf /var/cache/apk/* /tmp/* /var/tmp/*

# Pre-cache a few common dependencies to speed up first builds
RUN cargo search lazy_static
