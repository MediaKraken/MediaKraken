FROM debian:buster-slim

# Add start script
ADD chunkserver.sh /usr/sbin/chunkserver.sh

RUN apt-get update && apt-get install -y wget gnupg2 gettext-base \
  && wget -O - http://ppa.moosefs.com/moosefs.key 2>/dev/null | apt-key add - 2>/dev/null \
  && echo "deb http://ppa.moosefs.com/3.0.117/apt/debian/buster buster main" > /etc/apt/sources.list.d/moosefs.list \
  && apt-get update && apt-get install -y moosefs-chunkserver \
  && chown root:root /usr/sbin/chunkserver.sh ; chmod 700 /usr/sbin/chunkserver.sh ; mkdir -p /mnt/hdd0

# Expose ports
EXPOSE 9422

CMD ["chunkserver.sh"]