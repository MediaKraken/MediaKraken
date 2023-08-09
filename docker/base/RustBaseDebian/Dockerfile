# don't use the slim version as it's just a stage one image
# as too much stuff is missing causing stuff later to not compile
FROM rust:1.71.1-bookworm
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install libudev-dev wget ca-certificates libc-bin musl-tools clang -y \
  && update-ca-certificates \
  && wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.12.0/wait \
  && chmod +x /wait

# docker run -d --name redis-stack -p 6379:6379 -p 8001:8001 redis/redis-stack:latest
RUN wget https://github.com/mozilla/sccache/releases/download/v0.5.4/sccache-v0.5.4-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.5.4-x86_64-unknown-linux-musl.tar.gz \
    && mv sccache-v0.5.4-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache

# for layer cache later in build
RUN cargo install cargo-chef

ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_REDIS="redis://mksccache:6379/mksccache"
ENV RUSTFLAGS="-C target-feature=-crt-static"

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
    "${USER}" \
    && cargo search lazy_static
