# Download base image
FROM alpine:3.19.1

LABEL author="Quinn D Granfor, spootdev@gmail.com"

#   && apk add --no-cache postgresql-citus --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community \
RUN apk update && apk add --no-cache bash su-exec tzdata zstd icu-data-full nss_wrapper \
  && apk add --no-cache postgresql-citus postgresql16-contrib

ADD https://raw.githubusercontent.com/postgres/postgres/master/src/backend/utils/misc/postgresql.conf.sample /usr/local/share/postgresql/postgresql.conf.sample
COPY docker-entrypoint.sh /usr/local/bin/
COPY docker-entrypoint-initdb.d /docker-entrypoint-initdb.d

# make the sample config easier to munge (and "correct by default")
RUN set -eux; \
  sed -ri "s!^#?(listen_addresses)\s*=\s*\S+.*!\1 = '*'!" /usr/local/share/postgresql/postgresql.conf.sample; \
  grep -F "listen_addresses = '*'" /usr/local/share/postgresql/postgresql.conf.sample \
  && mkdir -p /var/run/postgresql && chown -R postgres:postgres /var/run/postgresql && chmod 3777 /var/run/postgresql

ENV PGDATA /var/lib/postgresql/data
# this 777 will be replaced by 700 at runtime (allows semi-arbitrary "--user" values)
RUN mkdir -p "$PGDATA" && chown -R postgres:postgres "$PGDATA" && chmod 1777 "$PGDATA" \
  && chmod +x /usr/local/bin/docker-entrypoint.sh

VOLUME /var/lib/postgresql/data

ENTRYPOINT ["docker-entrypoint.sh"]

STOPSIGNAL SIGINT
EXPOSE 5432
CMD ["postgres"]