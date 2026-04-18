# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mksharescanner
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mksharescanner AS cargo-build-mksharescanner
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mksharescanner /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mksharescanner /mediakraken/target/mksharescanner

####################################################################################################
## Final image
####################################################################################################
FROM alpine:3.20

# Install nmap (SYN + UDP scans need raw sockets) and rustscan.
# rustscan lives in edge/testing.
RUN apk add --no-cache nmap nmap-scripts libcap \
  && apk add --no-cache rustscan --repository=https://dl-cdn.alpinelinux.org/alpine/edge/testing \
  && setcap cap_net_raw,cap_net_admin,cap_net_bind_service=+eip /usr/bin/nmap \
  && addgroup -S mediakraken \
  && adduser -S -G mediakraken mediakraken \
  && rm -rf /var/cache/apk/*

COPY --from=cargo-build-mksharescanner /wait /wait
COPY --from=cargo-build-mksharescanner /mediakraken/target/mksharescanner /mksharescanner

USER mediakraken:mediakraken
