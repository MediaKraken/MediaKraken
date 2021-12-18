FROM rust:1.57 as cargo-build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=mediakraken
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /mediakraken

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM busybox:1.34.1-uclibc
RUN wget -O /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait &&\
    chmod +x /wait
# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/sh /sh

WORKDIR /mediakraken

# Copy our build
COPY --from=cargo-build /mediakraken/target/x86_64-unknown-linux-musl/release/mkdownload ./

# Use an unprivileged user.
USER mediakraken:mediakraken
