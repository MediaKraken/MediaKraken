FROM mediakraken/mkbase_rust:dev as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/x86_64-unknown-linux-musl/.; \
  else RUSTFLAGS=-Clinker=musl-gcc cargo build --target=x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/debug/mk* /mediakraken/target/x86_64-unknown-linux-musl/.; \
  fi

####################################################################################################
## Final image
####################################################################################################
FROM busybox:1.36.0-musl
RUN wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait &&\
  chmod +x /wait
# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/sh /sh

WORKDIR /mediakraken

# Copy our build
COPY --from=cargo-build /mediakraken/target/x86_64-unknown-linux-musl/mkcron ./

# Use an unprivileged user.
USER mediakraken:mediakraken
