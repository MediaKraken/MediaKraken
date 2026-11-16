# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkmediascanner
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mkmediascanner AS cargo-build-mkmediascanner
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mkmediascanner /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /

# Install native Alpine packages (manages all shared libraries correctly for musl)
RUN apk update && apk add --no-cache cifs-utils nfs-utils libsmbclient samba-client \
 && rm -rf /var/cache/apk/*

# Import from builder.
COPY --from=cargo-build-mkmediascanner /etc/passwd /etc/passwd
COPY --from=cargo-build-mkmediascanner /etc/group /etc/group
COPY --from=cargo-build-mkmediascanner /wait /wait

# Copy our build binary
COPY --from=cargo-build-mkmediascanner /mediakraken/target/mkmediascanner /mkmediascanner

# Use an unprivileged user.
USER mediakraken:mediakraken