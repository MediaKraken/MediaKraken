FROM rust:1.66.0 as cargo-build

ARG BRANCHTAG

RUN apt-get update \
  && apt-get install --no-install-recommends musl-tools -y \
  && rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/mkapp

COPY Cargo.toml Cargo.toml

RUN mkdir src/ \
  && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
  && RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl \
  && rm -f target/x86_64-unknown-linux-musl/release/deps/mkapp*

COPY . .

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl \
  && cp /usr/src/mkapp/target/x86_64-unknown-linux-musl/release/mk* /usr/src/mkapp/target/x86_64-unknown-linux-musl/.; \
  else RUSTFLAGS=-Clinker=musl-gcc cargo build --target=x86_64-unknown-linux-musl \
  && cp /usr/src/mkapp/target/x86_64-unknown-linux-musl/debug/mk* /usr/src/mkapp/target/x86_64-unknown-linux-musl/.; \
  fi

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:3.15.0

RUN addgroup -g 1000 mkapp \
  && adduser -D -s /bin/sh -u 1000 -G mkapp mkapp

WORKDIR /mediakraken

COPY --from=cargo-build /usr/src/mkapp/target/x86_64-unknown-linux-musl/mkapp .

RUN chown mkapp:mkapp mkapp

USER mkapp

CMD ["./mkapp"]