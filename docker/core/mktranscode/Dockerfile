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
RUN ldd /mediakraken/target/mktranscode | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

FROM mediakraken/mkbase_alpine_ffmpeg:dev as ffmpeg-build

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-3.18.2-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build /mediakraken/deps/* /

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=ffmpeg-build /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=ffmpeg-build /usr/local/bin/ffprobe /usr/local/bin/ffprobe
COPY --from=ffmpeg-build /usr/local/lib /usr/local/lib
COPY --from=cargo-build /wait /wait

# Copy our build
COPY --from=cargo-build /mediakraken/target/mktranscode /mktranscode

# Use an unprivileged user.
# USER mediakraken:mediakraken
