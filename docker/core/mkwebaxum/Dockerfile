# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_debian:"${BRANCHTAG}" as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then echo "Building Release" && cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else echo "Building Debug" && cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi

####################################################################################################
## Final image
####################################################################################################
# FROM gcr.io/distroless/base-debian11 as runtime
FROM debian:bullseye-20230522-slim AS runtime
WORKDIR /mediakraken

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /wait /wait
COPY --from=cargo-build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build /mediakraken/templates /mediakraken/templates
COPY --from=cargo-build /mediakraken/static /mediakraken/static

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkwebapp /mediakraken/mkwebapp

# Use an unprivileged user.
# USER mediakraken:mediakraken
