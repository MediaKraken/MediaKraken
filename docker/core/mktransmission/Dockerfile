# https://github.com/rlesouef/alpine-transmission

FROM alpine:3.21.3

LABEL maintainer="Open Source Services [opensourceservices.fr]"

COPY src/ .

RUN apk add --no-cache \
  busybox==1.37.0-r12 \
  transmission-daemon==4.0.6-r0 \
  && mkdir -p /transmission/downloads \
  && mkdir -p /transmission/incomplete \
  && mkdir -p /etc/transmission-daemon \
  && chmod +x /start-transmission.sh

VOLUME ["/etc/transmission-daemon"]
VOLUME ["/transmission/downloads"]
VOLUME ["/transmission/incomplete"]

EXPOSE 9091 51413/tcp 51413/udp

ENV USERNAME admin
ENV PASSWORD metaman

CMD ["/start-transmission.sh"]
