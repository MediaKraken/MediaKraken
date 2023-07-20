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

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-20230703-slim AS runtime

ADD https://github.com/yt-dlp/yt-dlp/releases/download/2023.01.06/yt-dlp_linux /yt-dlp
RUN chmod a+rx /yt-dlp \
  && apt-get update && apt install -y python3-pip \
  && rm -rf /var/lib/apt/lists/* \
  && pip3 install dosage==3.0 cssselect==1.2.0 --break-system-packages

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build /wait /wait

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkdownload /mkdownload

# Use an unprivileged user.
# USER mediakraken:mediakraken <- removed due throwing stuff in /mediakraken/static
