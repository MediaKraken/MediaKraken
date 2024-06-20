# Download base image
FROM alpine:3.20.0

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the image for pgbouncer"

RUN apk add --no-cache \
  busybox==1.36.1-r28 \
  c-ares==1.28.1-r0 \
  libevent==2.1.12-r7 \
  openssl==3.3.1-r0 \
  postgresql16-client==16.3-r0 \
  && apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/community \
  pgbouncer==1.22.1-r0 \
  && mkdir /run/postgresql \
  && chmod 777 /run/postgresql

#  && adduser -DHs /sbin/nologin metaman \

WORKDIR /
COPY wait-for-it-ash-busybox130.sh ./
COPY entrypoint.sh ./
RUN chmod +x ./*.sh
EXPOSE 6432
ENTRYPOINT ["./entrypoint.sh"]
