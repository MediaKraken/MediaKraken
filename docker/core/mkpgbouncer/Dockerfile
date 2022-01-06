# Download base image
FROM alpine:3.14.2

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the image for pgbouncer"

ARG ALPMIRROR

RUN sed -i "s/dl-cdn.alpinelinux.org/${ALPMIRROR}/" /etc/apk/repositories \
  && apk add --no-cache \
  busybox==1.33.1-r3 \
  c-ares==1.17.2-r0 \
  libevent==2.1.12-r2 \
  openssl-1.1.1l-r0 \
  pgbouncer==1.15.0-r0 \
  postgresql-client-13.5-r0 \
  && mkdir /run/postgresql \
  && chmod 777 /run/postgresql

#  && adduser -DHs /sbin/nologin metaman \

WORKDIR /
COPY wait-for-it-ash-busybox130.sh ./
COPY entrypoint.sh ./
RUN chmod +x ./*.sh
EXPOSE 6432
ENTRYPOINT ["./entrypoint.sh"]
