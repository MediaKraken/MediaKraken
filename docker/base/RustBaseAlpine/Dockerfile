FROM rust:1.89.0-alpine3.22
RUN apk update && \
  apk add alpine-sdk sqlite-dev openssl-dev wget ca-certificates \
  nmap postgresql16-client libsmbclient samba-dev libnfs-dev \
  && update-ca-certificates \
  && wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.12.1/wait \
  && chmod +x /wait

# docker run -d --name redis-stack -p 6379:6379 -p 8001:8001 redis/redis-stack:latest
RUN wget https://github.com/mozilla/sccache/releases/download/v0.9.1/sccache-v0.9.1-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.9.1-x86_64-unknown-linux-musl.tar.gz \
    && mv sccache-v0.9.1-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache

# for layer cache later in build
RUN cargo install cargo-chef

# ENV RUSTC_WRAPPER=/usr/local/bin/sccache
# ENV SCCACHE_REDIS="redis://mksccache:6379/mksccache"
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Create appuser
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "1000" \
  "mediakraken" \
&& cargo search lazy_static
