# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mkdownload
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY Cargo.toml Cargo.toml
COPY .cargo .cargo
COPY mk_lib_filler .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Cache/Build from Chef
####################################################################################################
FROM planner-mkdownload AS cargo-build-mkdownload
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mkdownload /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.;
RUN ldd /mediakraken/target/mkdownload | tr -s '[:blank:]' '\n' | grep '^/' | \
  xargs -I % sh -c 'mkdir -p $(dirname /mediakraken/deps%); cp % /mediakraken/deps%;'

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build-mkdownload /mediakraken/deps/* /

ADD https://github.com/yt-dlp/yt-dlp/releases/download/2023.07.06/yt-dlp_linux /yt-dlp
RUN chmod a+rx /yt-dlp \
 && apk update && apk add --no-cache py3-pip libssl3 libcrypto3 \
 && rm -rf /var/cache/apt/* \
 && pip3 install --no-cache-dir --trusted-host mksonatype -i http://mksonatype:8081/repository/pypi/simple dosage==3.0 cssselect==1.2.0 --break-system-packages

# Import from builder.
# COPY --from=cargo-build-mkdownload /etc/passwd /etc/passwd
# COPY --from=cargo-build-mkdownload /etc/group /etc/group
COPY --from=cargo-build-mkdownload /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=cargo-build-mkdownload /wait /wait

# Copy our build
COPY --from=cargo-build-mkdownload /mediakraken/target/mkdownload /mkdownload

# Use an unprivileged user.
# USER mediakraken:mediakraken <- removed due throwing stuff in /mediakraken/static
