# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_debian:"${BRANCHTAG}" as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN apt install libcurl4 libnghttp2-14

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-20230522-slim AS runtime

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build /wait /wait
COPY --from=cargo-build /usr/lib/x86_64-linux-gnu/libcurl.so.4.7.0 /usr/lib/x86_64-linux-gnu/libcurl.so.4
COPY --from=cargo-build /usr/lib/x86_64-linux-gnu/libnghttp2.so.14.20.1 /usr/lib/x86_64-linux-gnu/libnghttp2.so.14

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkmetadata /mkmetadata

# Use an unprivileged user.
# USER mediakraken:mediakraken
