# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_debian:"${BRANCHTAG}" as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner AS cargo-build

COPY --from=planner recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi

FROM mediakraken/mkbase_debian_ffmpeg:dev as ffmpeg-build

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-20230522-slim AS runtime

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
