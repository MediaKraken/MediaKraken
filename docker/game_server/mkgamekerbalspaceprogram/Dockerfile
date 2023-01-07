FROM mono:latest

LABEL maintainer="Jason Rivers <docker@jasonrivers.co.uk>"

RUN apt-get update && apt-get install -y \
	wget \
	unzip \
	&& apt-get clean \
    && mkdir -p /data/ksp \
	&& useradd -u 1000 -s /bin/bash -d /data/ksp ksp \
	&& chown ksp:ksp /data/ksp

COPY ksp.sh /usr/local/bin/ksp

EXPOSE 6702

USER ksp

ENV SERVER_REMOTE_FILE https://d-mp.org/builds/release/v0.3.8.3/DMPServer.zip

RUN cd /data/ksp && wget ${SERVER_REMOTE_FILE} && unzip DMPServer.zip \
  && mkdir -p /data/ksp/DMPServer/Universe && mkdir -p /data/ksp/config  \
  && mkdir -p /data/ksp/DMPServer/logs && mkdir -p /data/ksp/kspconfig

COPY config/*.txt /data/ksp/kspconfig/

VOLUME [ "/data/ksp/DMPServer/Universe", "/data/ksp/config", "/data/ksp/DMPServer/logs" ]
WORKDIR /data/ksp

CMD ["ksp"]
