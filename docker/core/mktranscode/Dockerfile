# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mktranscode

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mktranscode AS cargo-build-mktranscode

COPY --from=planner-mktranscode recipe.json recipe.json

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

####################################################################################################
## ffmpeg image
####################################################################################################
FROM cargo-build-mktranscode AS cargo-build-ffmpeg

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig \
    SRC=/usr/local

ARG FDKAAC_VERSION=2.0.2
#ARG X264_VERSION=20191217-2245-stable
ARG X265_VERSION=3.4
ARG FFMPEG_VERSION="6.0"
ARG OPENMPT_VERSION=0.6.7+release.makefile

COPY scripts /mediakraken
RUN mkdir /mediakraken/install \
  && mv /mediakraken/vid.stab-1.1.0 /mediakraken/install/vid.stab

RUN buildDeps="autoconf \
        automake \
        bash \
        binutils \
        bzip2 \
        cmake \
        curl \
        coreutils \
        g++ \
        gcc \
        git \
        gnupg \
        gnutls-dev \
        fontconfig-dev \
        freetype-dev \
        lame-dev \
        libass-dev \
        libmodplug-dev \
        libogg-dev \
        rtmpdump-dev \
        libtheora-dev \
        libtool \
        libva-dev \
        libvdpau-dev \
        libvorbis-dev \
        libvpx-dev \
        libwebp-dev \
        make \
        nasm \
        openjpeg-dev \
        opus-dev \
        python3-dev \
        samba-dev \
        speex-dev \
        soxr-dev \
        tar \
        xvidcore-dev \
        xz \
        yasm \
        zlib-dev" && \
    export MAKEFLAGS="-j$(($(grep -c ^processor /proc/cpuinfo) + 1))" && \
    apk add --no-cache ${buildDeps} libgcc libstdc++ \
    ca-certificates busybox && \
    DIR=$(mktemp -d) && cd ${DIR} && \
    git clone https://code.videolan.org/videolan/x264.git && \
    cd x264 && \
    ./configure --prefix="${SRC}" --bindir="${SRC}/bin" --enable-pic --enable-shared --disable-cli && \
    make -j`getconf _NPROCESSORS_ONLN` && \
    make install && \
    DIR=$(mktemp -d) && cd ${DIR} && \
    curl -L --insecure https://github.com/videolan/x265/archive/refs/tags/${X265_VERSION}.tar.gz | \    
    tar -zx --strip-components=1 && \
    cd build/linux && \
    cmake -G "Unix Makefiles" ../../source && \
    make -j`getconf _NPROCESSORS_ONLN` && \
    make install && \
    DIR=$(mktemp -d) && cd ${DIR} && \
    curl -L --insecure https://github.com/mstorsjo/fdk-aac/archive/refs/tags/v${FDKAAC_VERSION}.tar.gz | \
    tar -zx --strip-components=1 && \
    autoreconf -fiv && \
    ./configure --prefix="${SRC}" --disable-static --datadir="${DIR}" && \
    make -j`getconf _NPROCESSORS_ONLN` && \
    make install && \
    DIR=$(mktemp -d) && cd ${DIR} && \
    curl -L --insecure https://lib.openmpt.org/files/libopenmpt/src/libopenmpt-${OPENMPT_VERSION}.tar.gz | \
    tar -zx --strip-components=1 && \
    make EXAMPLES=0 && \
    make install && \
    chmod +x /mediakraken/*.sh && \
    /mediakraken/kvazaar.sh && \
    /mediakraken/openh264.sh && \
    /mediakraken/vidstab.sh && \
    # nvenc headers for ffmpeg
    git clone https://git.videolan.org/git/ffmpeg/nv-codec-headers.git && \
    cd nv-codec-headers && \
    make -j`getconf _NPROCESSORS_ONLN` && \
    make install && \
    DIR=$(mktemp -d) && cd ${DIR} && \
    curl -LO --insecure http://ffmpeg.org/releases/ffmpeg-${FFMPEG_VERSION}.tar.gz && \
    tar -zx --strip-components=1 -f ffmpeg-${FFMPEG_VERSION}.tar.gz && \
   ./configure \
    --target-os=linux \
        --bindir="${SRC}/bin" \
        --extra-libs=-ldl \
        --extra-cflags="-I${SRC}/include" \
        --extra-ldflags="-L${SRC}/lib" \
        --extra-cflags="-I/usr/local/cuda-10.0/include/" \
        --extra-ldflags=-L/usr/local/cuda-10.0/lib64/ \
        --prefix="${SRC}" \
        --enable-avfilter \
        --enable-swresample \
        --enable-fontconfig \
        --enable-gpl \
        --enable-libass \
        --enable-libfdk_aac \
        --enable-fontconfig \
        --enable-libfreetype \
        --enable-libkvazaar \
        --enable-libmodplug \
        --enable-libmp3lame \
        --enable-libopenh264 \
        --enable-libopenjpeg \
        --enable-libopenmpt \
        --enable-libopus \
        --enable-librtmp \
        --enable-libsoxr \
        --enable-libsmbclient \
        --enable-libspeex \
        --enable-libtheora \
        --enable-libwebp \
        --enable-vaapi \
        --enable-libvidstab \
        --enable-libvorbis \
        --enable-libvpx \
        --enable-libx264 \
        --enable-libx265 \
        --enable-libxvid \
        --enable-nonfree \
        --enable-postproc \
        --enable-pthreads \
        --enable-shared \
        --enable-small \
        --enable-vdpau \
        --enable-version3 \
        --enable-zlib \
        --disable-debug \
        --disable-doc \
        --disable-static \
        --disable-ffplay && \
    make -j`getconf _NPROCESSORS_ONLN` && \
    make install && \
    hash -r

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-3.18.2-x86_64.tar.gz /
# deps has full /usr/lib/x86_64-linux-gnu path.....hence copy to /
COPY --from=cargo-build-mktranscode /mediakraken/deps/* /

RUN apk update && apk add --no-cache cifs-utils nfs-utils \
 && rm -rf /var/cache/apt/*

# Import from builder.
# COPY --from=cargo-build-mktranscode /etc/passwd /etc/passwd
# COPY --from=cargo-build-mktranscode /etc/group /etc/group
COPY --from=cargo-build-ffmpeg /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=cargo-build-ffmpeg /usr/local/bin/ffprobe /usr/local/bin/ffprobe
# COPY --from=ffmpeg-build-mktranscode /usr/local/lib /usr/local/lib
COPY --from=cargo-build-mktranscode /wait /wait

# Copy our build
COPY --from=cargo-build-mktranscode /mediakraken/target/mktranscode /mktranscode

# Use an unprivileged user.
# USER mediakraken:mediakraken
