# https://github.com/rlesouef/alpine-transmission

FROM alpine:3.19.0

LABEL maintainer="Open Source Services [opensourceservices.fr]"

COPY src/ .

RUN apk add --no-cache \
  busybox==1.36.1-r15 \
  transmission-daemon==4.0.5-r0 \
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
