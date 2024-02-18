# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkrabbitconsume
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

# COPY Cargo.toml Cargo.toml
# COPY .cargo .cargo
# RUN mkdir /mediakraken/src
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mkrabbitconsume AS cargo-build-mkrabbitconsume
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mkrabbitconsume /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.;
RUN ldd /mediakraken/target/mkrabbitconsume | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build-mkrabbitconsume /mediakraken/deps/* /

# Import from builder.
# COPY --from=cargo-build-mkrabbitconsume /etc/passwd /etc/passwd
# COPY --from=cargo-build-mkrabbitconsume /etc/group /etc/group
COPY --from=cargo-build-mkrabbitconsume /wait /wait

# Copy our build
COPY --from=cargo-build-mkrabbitconsume /mediakraken/target/mkrabbitconsume /mkrabbitconsume

# Use an unprivileged user.
# USER mediakraken:mediakraken
