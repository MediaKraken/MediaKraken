FROM mediakraken/mkbase_rust_alpine:dev as test-planner-mkwebapp
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0
#ENV RUSTFLAGS="--Awarnings"

# COPY Cargo.toml Cargo.toml
# COPY .cargo .cargo
# RUN mkdir /mediakraken/src
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################cargo 
## Cache/Build from Chef
####################################################################################################
FROM test-planner-mkwebapp as test-cargo-build-mkwebapp
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0
#ENV RUSTFLAGS="--Awarnings"

COPY --from=test-planner-mkwebapp /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.;
RUN ldd /mediakraken/target/mkwebapp | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=test-cargo-build-mkwebapp /mediakraken/deps/* /

WORKDIR /mediakraken

RUN apk update && apk add --no-cache musl-locales lang libssl3 libcrypto3 cifs-utils nfs-utils libsmbclient \
 && rm -rf /var/cache/apt/*

# RUN apt-get update && apt-get install -y locales locales-all \
#   && rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=test-cargo-build-mkwebapp /wait /wait
COPY --from=test-cargo-build-mkwebapp /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy our build
COPY --from=test-cargo-build-mkwebapp /mediakraken/target/mkwebapp /mediakraken/mkwebapp

# Use an unprivileged user.
# USER mediakraken:mediakraken

EXPOSE 3000
CMD ["/mediakraken/mkwebapp"]
