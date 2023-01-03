# https://github.com/rlesouef/alpine-transmission

FROM alpine:3.16.2

LABEL maintainer="Open Source Services [opensourceservices.fr]"

RUN apk add --no-cache \
  busybox==1.35.0-r17 \
  transmission-daemon==3.00-r6 \
  && mkdir -p /transmission/downloads \
  && mkdir -p /transmission/incomplete \
  && mkdir -p /etc/transmission-daemon

COPY src/ .

VOLUME ["/etc/transmission-daemon"]
VOLUME ["/transmission/downloads"]
VOLUME ["/transmission/incomplete"]

EXPOSE 9091 51413/tcp 51413/udp

ENV USERNAME admin
ENV PASSWORD metaman

RUN chmod +x /start-transmission.sh
CMD ["/start-transmission.sh"]
