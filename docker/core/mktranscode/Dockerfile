# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mktranscode
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## ffmpeg image (Shared Libraries in /usr/lib)
####################################################################################################
FROM planner-mktranscode AS cargo-build-ffmpeg

ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig \
    SRC=/usr \
    DEBIAN_FRONTEND="noninteractive" \
    CARGO_INCREMENTAL=0

ARG FDKAAC_VERSION=2.0.3
ARG FFMPEG_VERSION=9.0.2
ARG KVAZAAR=2.3.2
ARG LIBMFX=22.5.4
ARG LIBVMAF=3.0.0
ARG LIBVPL=2.11.0
ARG ONEVPL=24.1.5
ARG OPENCOREAMR=0.1.6
ARG OPENH264=2.6.0
ARG OPENMPT_VERSION=0.7.7+release.makefile

RUN buildDeps="aom-dev \
        autoconf \
        automake \
        bash \
        binutils \
        bzip2 \
        cmake \
        curl \
        coreutils \
        diffutils \
        doxygen \
        g++ \
        gcc \
        git \
        gnupg \
        gnutls-dev \
        gperf \
        fontconfig-dev \
        freetype-dev \
        intel-media-driver-dev \
        intel-gmmlib-dev \
        lame-dev \
        libass-dev \
        libgomp \
        libmodplug-dev \
        libogg-dev \
        rtmpdump-dev \
        libdrm-dev \
        libtheora-dev \
        libtool \
        libva-dev \
        libvdpau-dev \
        libvorbis-dev \
        libvpx-dev \
        libwebp-dev \
        make \
        nasm \
        ninja \
        ninja-build \
        opencl-icd-loader-dev \
        openjpeg-dev \
        opus-dev \
        python3-dev \
        py3-pip \
        samba-dev \
        speex-dev \
        soxr-dev \
        tar \
        x265-dev \
        xvidcore-dev \
        xxd \
        xz \
        yasm \
        zlib-dev" && \
    export MAKEFLAGS="-j$(($(grep -c ^processor /proc/cpuinfo) + 1))" && \
    apk add --no-cache ${buildDeps} libgcc libstdc++ \
    ca-certificates busybox && \
    pip3 install --no-cache-dir meson cmake --break-system-packages

    # x264 (Shared)
    RUN echo "Begin: x264" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      git clone --depth 1 https://code.videolan.org/videolan/x264.git && \
      cd x264 && \
      ./configure --prefix="${SRC}" --bindir="${SRC}/bin" --enable-pic --enable-shared --disable-cli && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # fdk-aac (Shared)
    RUN echo "Begin: fdk-aac" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/mstorsjo/fdk-aac/archive/refs/tags/v${FDKAAC_VERSION}.tar.gz | \
      tar -zx --strip-components=1 && \
      autoreconf -fiv && \
      ./configure --prefix="${SRC}" --disable-static --enable-shared --datadir="${DIR}" && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # openmpt
    RUN echo "Begin: openmpt" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://lib.openmpt.org/files/libopenmpt/src/libopenmpt-${OPENMPT_VERSION}.tar.gz | \
      tar -zx --strip-components=1 && \
      make -j`getconf _NPROCESSORS_ONLN` EXAMPLES=0 PREFIX="${SRC}" && \
      make install PREFIX="${SRC}"

    # kvazaar
    RUN echo "Begin: kvazaar" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/ultravideo/kvazaar/archive/refs/tags/v${KVAZAAR}.tar.gz | \
      tar -zx --strip-components=1 && \
      ./autogen.sh && \
      ./configure --prefix="${SRC}" --enable-shared && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # OPENCOREAMR (Shared)
    RUN echo "Begin: opencoreamr" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf http://downloads.sourceforge.net/project/opencore-amr/opencore-amr/opencore-amr-${OPENCOREAMR}.tar.gz | \
      tar -zx --strip-components=1 && \
      ./configure --prefix="${SRC}" --disable-static --enable-shared && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # openh264
    RUN echo "Begin: openh264" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/cisco/openh264/archive/refs/tags/v${OPENH264}.tar.gz | \
      tar -zx --strip-components=1 && \
      make -j`getconf _NPROCESSORS_ONLN` PREFIX="${SRC}" && \
      make install PREFIX="${SRC}"

    # LIBMFX
    RUN echo "Begin: libmfx" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/Intel-Media-SDK/MediaSDK/archive/refs/tags/intel-mediasdk-${LIBMFX}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake \
        -DCMAKE_INSTALL_PREFIX=/usr \
        -DCMAKE_INSTALL_LIBDIR=/usr/lib \
        -DBUILD_SAMPLES=OFF \
        -DENABLE_X11_DRI3=ON \
        -DBUILD_DISPATCHER=OFF \
        -DBUILD_TUTORIALS=OFF \
        . && \ 
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # LIBVMAF
    RUN echo "Begin: libvmaf" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/Netflix/vmaf/archive/refs/tags/v${LIBVMAF}.tar.gz | \
      tar -zx --strip-components=1 && \
      cd libvmaf && \
      meson setup \
        --prefix=/usr --libdir=/usr/lib \
        --buildtype release \
        build && \
      ninja -vC build && \
      ninja -vC build install

    # LIBVPL
    RUN echo "Begin: libvpl" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/oneapi-src/oneVPL/archive/refs/tags/v${LIBVPL}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=/usr/lib . && \ 
      cmake --build . --config Release && \
      cmake --build . --config Release --target install

    # ONEVPL
    RUN echo "Begin: onevpl" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/oneapi-src/oneVPL-intel-gpu/archive/refs/tags/intel-onevpl-${ONEVPL}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake \
        -DCMAKE_INSTALL_PREFIX=/usr \
        -DCMAKE_INSTALL_LIBDIR=/usr/lib \
        . && \ 
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # nvenc headers for ffmpeg
    RUN echo "Begin: nvenc" && \    
      git clone --depth 1 https://git.videolan.org/git/ffmpeg/nv-codec-headers.git && \
      cd nv-codec-headers && \
      make PREFIX=/usr install

    # AMD AMF headers for ffmpeg
    RUN echo "Begin: amf-headers" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      git clone --depth 1 https://github.com/GPUOpen-LibrariesAndSDKs/AMF.git && \
      mkdir -p /usr/include/AMF && \
      cp -r AMF/amf/public/include/* /usr/include/AMF/

    # ffmpeg itself (Shared)
    RUN echo "Begin: ffmpeg" && \    
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -LO --insecure http://ffmpeg.org/releases/ffmpeg-${FFMPEG_VERSION}.tar.gz && \
      tar -zx --strip-components=1 -f ffmpeg-${FFMPEG_VERSION}.tar.gz && \
      ./configure \
      --target-os=linux \
          --bindir="${SRC}/bin" \
          --extra-libs="-ldl -lpthread" \
          --extra-cflags="-I${SRC}/include" \
          --extra-ldflags="-L${SRC}/lib" \
          --prefix="${SRC}" \
          --enable-avfilter \
          --enable-swresample \
          --enable-fontconfig \
          --enable-gpl \
          --enable-libaom \
          --enable-libass \
          --enable-libfdk_aac \
          --enable-libfreetype \
          --enable-libkvazaar \
          --enable-libmodplug \
          --enable-libmp3lame \
          --enable-libopencore-amrnb \
          --enable-libopencore-amrwb \          
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
          --enable-ffnvcodec \
          --enable-nvenc \
          --enable-cuvid \
          --enable-amf \
          --enable-vaapi \
          --enable-libvpl \
          --enable-libvmaf \          
          --enable-libvorbis \
          --enable-libvpl \          
          --enable-libvpx \
          --enable-libx264 \
          --enable-libx265 \
          --enable-libxvid \
          --enable-nonfree \
          --enable-pthreads \
          --enable-shared \
          --disable-static \
          --enable-small \
          --enable-vdpau \
          --enable-version3 \
          --enable-zlib \
          --disable-debug \
          --disable-doc \
          --disable-ffplay && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install && \
      hash -r

####################################################################################################
## Build from Chef
####################################################################################################
FROM planner-mktranscode AS cargo-build-mktranscode
WORKDIR /mediakraken
ENV CARGO_INCREMENTAL=0

COPY --from=planner-mktranscode /mediakraken/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-musl \
  && cp /mediakraken/target/x86_64-unknown-linux-musl/release/mk* /mediakraken/target/.;

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /

RUN apk update && apk add --no-cache cifs-utils nfs-utils \
 && apk add --no-cache \
      --repository=https://dl-cdn.alpinelinux.org/alpine/edge/main \
      --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community \
      --repository=https://dl-cdn.alpinelinux.org/alpine/edge/testing \
      python3 calibre \
 && rm -rf /var/cache/apk/*

# Import from builder.
COPY --from=cargo-build-mktranscode /etc/passwd /etc/passwd
COPY --from=cargo-build-mktranscode /etc/group /etc/group

# Copy ffmpeg binaries and the complete /usr/lib shared library closure directly into native paths
COPY --from=cargo-build-ffmpeg /usr/bin/ffmpeg /usr/bin/ffmpeg
COPY --from=cargo-build-ffmpeg /usr/bin/ffprobe /usr/bin/ffprobe
COPY --from=cargo-build-ffmpeg /usr/lib /usr/lib

COPY --from=cargo-build-mktranscode /wait /wait

# Copy our build
COPY --from=cargo-build-mktranscode /mediakraken/target/mktranscode /mktranscode

# Use an unprivileged user.
USER mediakraken:mediakraken