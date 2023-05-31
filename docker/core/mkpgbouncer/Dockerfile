# Download base image
FROM alpine:3.17.3

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the image for pgbouncer"

RUN apk add --no-cache \
  busybox==1.35.0-r29 \
  c-ares==1.19.1-r0 \
  libevent==2.1.12-r5 \
  openssl==3.0.8-r4 \
  pgbouncer==1.17.0-r1 \
  postgresql15-client==15.3-r0 \
  && mkdir /run/postgresql \
  && chmod 777 /run/postgresql

#  && adduser -DHs /sbin/nologin metaman \

WORKDIR /
COPY wait-for-it-ash-busybox130.sh ./
COPY entrypoint.sh ./
RUN chmod +x ./*.sh
EXPOSE 6432
ENTRYPOINT ["./entrypoint.sh"]
