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
FROM busybox:1.35.0-uclibc
RUN wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait &&\
  chmod +x /wait
# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/sh /sh
COPY --from=cargo-build /mediakraken/templates /mediakraken/templates
COPY --from=cargo-build /mediakraken/Rocket.toml /mediakraken/Rocket.toml

WORKDIR /mediakraken
RUN chown -R mediakraken: /mediakraken

# Copy our build
COPY --from=cargo-build /mediakraken/target/x86_64-unknown-linux-musl/mkwebapp ./

# Use an unprivileged user.
USER mediakraken:mediakraken
