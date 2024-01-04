# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkmetadata

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mkmetadata AS cargo-build-mkmetadata

COPY --from=planner-mkmetadata recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

# RUN apt install libcurl4 libnghttp2-14 librtmp1 libssh2-1 libpsl5 libldap-2.5-0 libbrotli1

RUN if [ "${BRANCHTAG}" = "dev" ]; \
  then cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.; \
  else cargo build --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/debug/mk* /mediakraken/target/.; \
  fi
RUN ldd /mediakraken/target/mkmetadata | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build-mkmetadata /mediakraken/deps/* /

RUN apk update && apk add --no-cache libssl1.1 libcrypto1.1 \
 && rm -rf /var/cache/apt/*

# Import from builder.
# COPY --from=cargo-build-mkmetadata /etc/passwd /etc/passwd
# COPY --from=cargo-build-mkmetadata /etc/group /etc/group
COPY --from=cargo-build-mkmetadata /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build-mkmetadata /wait /wait

# Copy our build
COPY --from=cargo-build-mkmetadata /mediakraken/target/mkmetadata /mkmetadata

# Use an unprivileged user.
# USER mediakraken:mediakraken
