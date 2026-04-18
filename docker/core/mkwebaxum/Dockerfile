# Pre-req: Use a consistent build arg for the base image
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkwebapp
WORKDIR /mediakraken

# 1. Generate the recipe (Only happens if Cargo.toml/lock changes)
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as cacher-mkwebapp
WORKDIR /mediakraken

# 2. Build dependencies only (This layer is cached heavily)
COPY --from=planner-mkwebapp /mediakraken/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

################################################################################
FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as builder-mkwebapp
WORKDIR /mediakraken

# 3. Copy cached dependencies from cacher
COPY --from=cacher-mkwebapp /mediakraken/target target
COPY --from=cacher-mkwebapp /usr/local/cargo /usr/local/cargo

# 4. Build actual source code
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Preparation for the final image
RUN cp /mediakraken/target/x86_64-unknown-linux-musl/release/mkwebapp /mediakraken/mkwebapp

# Dynamically gather shared library dependencies
RUN mkdir -p /mediakraken/deps && \
    ldd /mediakraken/mkwebapp | grep "=> /" | awk '{print $3}' | xargs -I % cp --parents % /mediakraken/deps

################################################################################
FROM alpine:latest
# Use 'alpine' instead of 'scratch' + 'ADD tar.gz' for better layer management 
# and built-in security updates.

WORKDIR /mediakraken

# Install runtime dependencies
RUN apk add --no-cache \
    musl-locales \
    libssl3 \
    libcrypto3 \
    cifs-utils \
    nfs-utils \
    libsmbclient

# Copy system files and certs
COPY --from=builder-mkwebapp /etc/passwd /etc/passwd
COPY --from=builder-mkwebapp /etc/group /etc/group
COPY --from=builder-mkwebapp /wait /wait
COPY --from=builder-mkwebapp /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy the binary and assets
COPY --from=builder-mkwebapp /mediakraken/mkwebapp /mediakraken/mkwebapp
COPY --from=builder-mkwebapp /mediakraken/templates ./templates
COPY --from=builder-mkwebapp /mediakraken/static ./static

# Copy the dynamically found libraries into the root
COPY --from=builder-mkwebapp /mediakraken/deps/ /

USER mediakraken:mediakraken