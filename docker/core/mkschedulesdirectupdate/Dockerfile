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

# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkschedulesdirectupdate /mkschedulesdirectupdate

# Use an unprivileged user.
USER mediakraken:mediakraken
