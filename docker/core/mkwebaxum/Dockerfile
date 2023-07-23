# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner AS cargo-build

COPY --from=planner recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN if [ "${BRANCHTAG}" = "dev" ]; \
  then cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.; \
  else cargo build --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/debug/mk* /mediakraken/target/.; \
  fi
RUN ldd /mediakraken/target/mkwebapp | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
# FROM gcr.io/distroless/base-debian11 as runtime
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-3.18.2-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build /mediakraken/deps/* /

WORKDIR /mediakraken

RUN apk update && apk add --no-cache musl-locales lang \
 && rm -rf /var/cache/apt/*

# RUN apt-get update && apt-get install -y locales locales-all \
#   && rm -rf /var/lib/apt/lists/*

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
