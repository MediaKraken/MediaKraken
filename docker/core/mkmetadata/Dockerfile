# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_debian:"${BRANCHTAG}" as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN apt install libcurl4 libnghttp2-14 librtmp1 libssh2-1 libpsl5 libldap-2.4-2 libbrotli1

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi
RUN ldd /mediakraken/target/mkmetadata | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname deps%); cp % deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-20230522-slim AS runtime

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build /wait /wait
# deps has full /usr/lib/x86 path.....hence copy to /
COPY --from=cargo-build /mediakraken/deps /

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkmetadata /mkmetadata

# Use an unprivileged user.
# USER mediakraken:mediakraken
