FROM debian:buster-slim

# Add start script
ADD metalogger.sh /usr/sbin/metalogger.sh

RUN apt-get update && apt-get install -y wget gnupg2 gettext-base \
  && wget -O - http://ppa.moosefs.com/moosefs.key 2>/dev/null | apt-key add - 2>/dev/null \
  && echo "deb http://ppa.moosefs.com/3.0.117/apt/debian/buster buster main" > /etc/apt/sources.list.d/moosefs.list \
  && apt-get update && apt-get install -y moosefs-metalogger \
  && chown root:root /usr/sbin/metalogger.sh ; chmod 700 /usr/sbin/metalogger.sh

CMD ["metalogger.sh"]