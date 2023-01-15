FROM mediakraken/mkbase_rust_alpine:dev as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi

####################################################################################################
## Final image
####################################################################################################
FROM alpine:3.17.1 AS runtime
RUN wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait &&\
  chmod +x /wait
# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/sh /sh

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkmetadatamame /mkmetadatamame

# Use an unprivileged user.
USER mediakraken:mediakraken
