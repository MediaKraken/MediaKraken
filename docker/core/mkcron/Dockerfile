# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkcron
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY Cargo.toml Cargo.toml
COPY .cargo .cargo
COPY mk_lib_filler .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Cache/Build from Chef
####################################################################################################
FROM planner-mkcron AS cargo-build-mkcron
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mkcron /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.;
RUN ldd /mediakraken/target/mkcron | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build-mkcron /mediakraken/deps/* /

# Import from builder.
# COPY --from=cargo-build-mkcron /etc/passwd /etc/passwd
# COPY --from=cargo-build-mkcron /etc/group /etc/group
COPY --from=cargo-build-mkcron /wait /wait

# Copy our build
COPY --from=cargo-build-mkcron /mediakraken/target/mkcron /mkcron

# Use an unprivileged user.
# USER mediakraken:mediakraken
