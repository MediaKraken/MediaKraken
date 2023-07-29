FROM debian:buster-slim

# Add start script
ADD master.sh /usr/sbin/master.sh

RUN apt-get update && apt-get install -y wget gnupg2 \
  && wget -O - http://ppa.moosefs.com/moosefs.key 2>/dev/null | apt-key add - 2>/dev/null \
  && echo "deb http://ppa.moosefs.com/3.0.117/apt/debian/buster buster main" > /etc/apt/sources.list.d/moosefs.list \
  && apt-get update && apt-get install -y moosefs-master \
  && chown root:root /usr/sbin/master.sh ; chmod 700 /usr/sbin/master.sh

# Expose ports master ports
EXPOSE 9419 9420 9421

CMD ["master.sh"]