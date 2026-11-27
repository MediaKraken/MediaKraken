FROM alpine:3.24.4

COPY src/ .

RUN apk add --no-cache \
  busybox==1.37.0-r31 \
  transmission-daemon==4.1.2-r0 \
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
