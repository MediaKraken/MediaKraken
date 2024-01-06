# outside of build step, hence have to re arg it below
ARG BRANCHTAG

FROM mediakraken/mkbase_rust_alpine:"${BRANCHTAG}" as planner-mktranscode

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

####################################################################################################
## ffmpeg image
####################################################################################################
FROM planner-mktranscode AS cargo-build-ffmpeg

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig \
    SRC=/usr/local \
    DEBIAN_FRONTEND="noninteractive"

# https://aomedia.googlesource.com/aom
ARG AOM=v3.8.0
# https://github.com/mstorsjo/fdk-aac
ARG FDKAAC_VERSION=2.0.3
# https://ffmpeg.org/download.html
ARG FFMPEG_VERSION=6.1.1
# FONTCONFIG=2.14.2 - 2.14.2-r3
# FREETYPE=2.12.1 - 2.13.0-r5
# TODO FRIBIDI=1.0.13  <- why is hebrew lang needed?
# https://github.com/intel/gmmlib
ARG GMMLIB=22.3.15
# https://github.com/intel/media-driver
ARG IHD=23.4.3
# https://github.com/ultravideo/kvazaar
ARG KVAZAAR=2.2.0
# LAME=3.100 - 3.100-r5
# LIBASS=0.17.1 - 0.17.1-r0
# https://dri.freedesktop.org/libdrm
ARG LIBDRM=2.4.119
# https://github.com/Intel-Media-SDK/MediaSDK
ARG LIBMFX=22.5.4
# LIBVA=2.18.0 - 2.18.0-r1
# LIBVDPAU=1.5 - 1.5-r1
# https://github.com/georgmartius/vid.stab
ARG LIBVIDSTAB=1.1.1
# https://github.com/Netflix/vmaf
ARG LIBVMAF=3.0.0
# https://github.com/oneapi-src/oneVPL
ARG LIBVPL=2.10.1
# NVCODEC=n12.0.16.0
# OGG=1.3.5 - 1.3.5-r4
# https://github.com/oneapi-src/oneVPL-intel-gpu/
ARG ONEVPL=23.4.3
# ttp://downloads.sourceforge.net/project/opencore-amr/opencore-amr
ARG OPENCOREAMR=0.1.6
# https://github.com/cisco/openh264
ARG OPENH264=2.4.0
# https://lib.openmpt.org/files/libopenmpt/src/
ARG OPENMPT_VERSION=0.7.3+release.makefile
# OPENJPEG=2.5.0 - 2.5.0-r3
# OPUS=1.3.1 - 1.4-r0
# THEORA=1.1.1 - 1.1.1-r17
# VORBIS=1.3.7 - 1.3.7-r1
# VPX=1.13.0 -1.13.0-r1
# https://bitbucket.org/multicoreware/x265_git
ARG X265=3.5
# XVID=1.3.7 - 1.3.7-r1

RUN buildDeps="autoconf \
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
        lame-dev \
        libass-dev \
        libgomp \
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
        xvidcore-dev \
        xxd \
        xz \
        yasm \
        zlib-dev" && \
    export MAKEFLAGS="-j$(($(grep -c ^processor /proc/cpuinfo) + 1))" && \
    apk add --no-cache ${buildDeps} libgcc libstdc++ \
    ca-certificates busybox && \
    pip3 install --no-cache-dir --trusted-host mksonatype -i http://mksonatype:8081/repository/pypi/simple meson cmake --break-system-packages

    # aom
    RUN echo "Begin: aom" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      git clone --branch ${AOM} --depth 1 https://aomedia.googlesource.com/aom && \
      cd aom && \
      mkdir -p aom_build && \
      cd aom_build && \
      cmake -DBUILD_STATIC_LIBS=0 .. && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # x264
    RUN echo "Begin: x264" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      git clone --depth 1 https://code.videolan.org/videolan/x264.git && \
      cd x264 && \
      ./configure --prefix="${SRC}" --bindir="${SRC}/bin" --enable-pic --enable-shared --disable-cli && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # fdk-aac
    RUN echo "Begin: fdk-aac" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/mstorsjo/fdk-aac/archive/refs/tags/v${FDKAAC_VERSION}.tar.gz | \
      tar -zx --strip-components=1 && \
      autoreconf -fiv && \
      ./configure --prefix="${SRC}" --disable-static --datadir="${DIR}" && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # openmpt
    RUN echo "Begin: openmpt" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://lib.openmpt.org/files/libopenmpt/src/libopenmpt-${OPENMPT_VERSION}.tar.gz | \
      tar -zx --strip-components=1 && \
      make -j`getconf _NPROCESSORS_ONLN` EXAMPLES=0 && \
      make install

    # kvazaar
    RUN echo "Begin: kvazaar" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/ultravideo/kvazaar/archive/refs/tags/v${KVAZAAR}.tar.gz | \
      tar -zx --strip-components=1 && \
      ./autogen.sh && \
      ./configure --enable-static && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # OPENCOREAMR
    RUN echo "Begin: opencoreamr" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf http://downloads.sourceforge.net/project/opencore-amr/opencore-amr/opencore-amr-${OPENCOREAMR}.tar.gz | \
      tar -zx --strip-components=1 && \
      ./configure --disable-static --enable-shared && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # openh264
    RUN echo "Begin: openh264" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/cisco/openh264/archive/refs/tags/v${OPENH264}.tar.gz | \
      tar -zx --strip-components=1 && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # vistab
    RUN echo "Begin: vistab" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://github.com/georgmartius/vid.stab/archive/refs/tags/v${LIBVIDSTAB}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake . && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # GMMLIB
    RUN echo "Begin: gmmlib" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/intel/gmmlib/archive/refs/tags/intel-gmmlib-${GMMLIB}.tar.gz | \
      tar -zx --strip-components=1 && \
      mkdir build && cd build && \
      cmake -DCMAKE_BUILD_TYPE=Release -DARCH=64 .. && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install && \
      strip -d /usr/local/lib/libigdgmm.so

    # LIBDRM
    RUN echo "Begin: libdrm" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://dri.freedesktop.org/libdrm/libdrm-${LIBDRM}.tar.xz | \
      tar -xJ --strip-components=1 && \
      meson setup --prefix=/usr --libdir=/usr/local/lib/x86_64-linux-gnu -Dvalgrind=disabled . build && \
      ninja -C build && \
      ninja -C build install && \
      strip -d /usr/local/lib/x86_64-linux-gnu/libdrm*.so

    # IHD
    RUN echo "Begin: ihd" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/intel/media-driver/archive/refs/tags/intel-media-${IHD}.tar.gz | \
      tar -zx --strip-components=1 && \
      mkdir build && cd build && \
      cmake -DLIBVA_DRIVERS_PATH=/usr/lib/x86_64-linux-gnu/dri/ .. && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install && \
      strip -d /usr/lib/x86_64-linux-gnu/dri/iHD_drv_video.so

    # LIBMFX
    RUN echo "Begin: libmfx" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/Intel-Media-SDK/MediaSDK/archive/refs/tags/intel-mediasdk-${LIBMFX}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake \
        -DCMAKE_INSTALL_PREFIX=/usr \
        -DCMAKE_INSTALL_LIBDIR=/usr/local/lib \
        -DBUILD_SAMPLES=OFF \
        -DENABLE_X11_DRI3=ON \
        -DBUILD_DISPATCHER=OFF \
        -DBUILD_TUTORIALS=OFF \
        . && \ 
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install && \
      strip -d \
        /usr/local/lib/libmfxhw64.so \
        /usr/local/lib/mfx/libmfx_*.so

    # LIBVMAF
    RUN echo "Begin: libvmaf" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/Netflix/vmaf/archive/refs/tags/v${LIBVMAF}.tar.gz | \
      tar -zx --strip-components=1 && \
      cd libvmaf && \
      meson setup \
        --prefix=/usr --libdir=/usr/local/lib \
        --buildtype release \
        build && \
      ninja -vC build && \
      ninja -vC build install

    # LIBVPL
    RUN echo "Begin: libvpl" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/oneapi-src/oneVPL/archive/refs/tags/v${LIBVPL}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake . && \ 
      cmake --build . --config Release && \
      cmake --build . --config Release --target install && \
      strip -d /usr/local/lib/libvpl.so

    # ONEVPL
    RUN echo "Begin: onevpl" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -Lf https://github.com/oneapi-src/oneVPL-intel-gpu/archive/refs/tags/intel-onevpl-${ONEVPL}.tar.gz | \
      tar -zx --strip-components=1 && \
      cmake \
        -DCMAKE_INSTALL_PREFIX=/usr \
        -DCMAKE_INSTALL_LIBDIR=/usr/local/lib \
        . && \ 
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install && \
      strip -d /usr/local/lib/libmfx-gen.so

    # x265
    RUN echo "Begin: x265" && \
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -L --insecure https://bitbucket.org/multicoreware/x265_git/downloads/x265_${X265}.tar.gz | \
      tar -zx --strip-components=1 && \
      cd build/linux && \
      ./multilib.sh && \
      make -j`getconf _NPROCESSORS_ONLN` -C 8bit install

    # nvenc headers for ffmpeg
    RUN echo "Begin: nvenc" && \    
      git clone --depth 1 https://git.videolan.org/git/ffmpeg/nv-codec-headers.git && \
      cd nv-codec-headers && \
      make -j`getconf _NPROCESSORS_ONLN` && \
      make install

    # ffmpeg itself
    RUN echo "Begin: ffmpeg" && \    
      DIR=$(mktemp -d) && cd ${DIR} && \
      curl -LO --insecure http://ffmpeg.org/releases/ffmpeg-${FFMPEG_VERSION}.tar.gz && \
      tar -zx --strip-components=1 -f ffmpeg-${FFMPEG_VERSION}.tar.gz && \
      ./configure \
      --target-os=linux \
          --bindir="${SRC}/bin" \
          --extra-libs=-ldl \
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
          --enable-fontconfig \
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
          --enable-vaapi \
          --enable-libvidstab \
          --enable-libvmaf \          
          --enable-libvorbis \
          --enable-libvpl \          
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
## Final image
####################################################################################################
FROM scratch
# otherwise throws file not found error
ADD alpine-minirootfs-x86_64.tar.gz /
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
