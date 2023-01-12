# Download base image
FROM alpine:3.17.1

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the image for pgbouncer"

RUN apk add --no-cache \
  busybox==1.35.0-r29 \
  c-ares==1.18.1-r1 \
  libevent==2.1.12-r5 \
  openssl==3.0.7-r2 \
  pgbouncer==1.17.0-r1 \
  postgresql15-client==15.1-r0 \
  && mkdir /run/postgresql \
  && chmod 777 /run/postgresql

#  && adduser -DHs /sbin/nologin metaman \

WORKDIR /
COPY wait-for-it-ash-busybox130.sh ./
COPY entrypoint.sh ./
RUN chmod +x ./*.sh
EXPOSE 6432
ENTRYPOINT ["./entrypoint.sh"]
