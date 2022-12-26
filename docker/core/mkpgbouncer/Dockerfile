# Download base image
FROM alpine:3.16.2

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the image for pgbouncer"

RUN apk add --no-cache \
  busybox==1.35.0-r17 \
  c-ares==1.18.1-r0 \
  libevent==2.1.12-r4 \
  openssl==1.1.1s-r0 \
  pgbouncer==1.17.0-r0 \
  postgresql14-client==14.5-r0 \
  && mkdir /run/postgresql \
  && chmod 777 /run/postgresql

#  && adduser -DHs /sbin/nologin metaman \

WORKDIR /
COPY wait-for-it-ash-busybox130.sh ./
COPY entrypoint.sh ./
RUN chmod +x ./*.sh
EXPOSE 6432
ENTRYPOINT ["./entrypoint.sh"]
