ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:${BRANCHTAG} AS planner
WORKDIR /mediakraken

# Copy manifests, .cargo registry config, and source targets for cargo metadata
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
FROM mediakraken/mkbase_rust_alpine:${BRANCHTAG} AS cacher
WORKDIR /mediakraken

ARG BRANCHTAG=dev
COPY --from=planner /mediakraken/recipe.json recipe.json

# Mount Cargo registry/git caches and the target directory across builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/mediakraken/target \
    if [ "${BRANCHTAG}" = "prod" ]; then \
        cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json; \
    else \
        cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json; \
    fi

################################################################################
FROM mediakraken/mkbase_rust_alpine:${BRANCHTAG} AS builder
WORKDIR /mediakraken

ARG BRANCHTAG=dev
COPY --from=planner /mediakraken/recipe.json recipe.json
COPY . .

# Reuse cached Cargo registry, git index, and target artifacts during compilation
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/mediakraken/target \
    if [ "${BRANCHTAG}" = "prod" ]; then \
        cargo build --release --target x86_64-unknown-linux-musl && \
        cp /mediakraken/target/x86_64-unknown-linux-musl/release/mkwebapp /mediakraken/mkwebapp; \
    else \
        cargo build --target x86_64-unknown-linux-musl && \
        cp /mediakraken/target/x86_64-unknown-linux-musl/debug/mkwebapp /mediakraken/mkwebapp; \
    fi

################################################################################
FROM alpine:latest AS runtime
WORKDIR /mediakraken

# Install runtime OS packages before copying changing application artifacts
RUN apk add --no-cache \
    musl-locales \
    libssl3 \
    libcrypto3 \
    cifs-utils \
    nfs-utils \
    libsmbclient \
    samba-client

# Copy static system metadata and CA bundle before volatile app files
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /wait /wait
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy frequently changing application binary and static assets last
COPY --from=builder /mediakraken/mkwebapp /mediakraken/mkwebapp
COPY --from=builder /mediakraken/templates ./templates
COPY --from=builder /mediakraken/static ./static

USER mediakraken:mediakraken