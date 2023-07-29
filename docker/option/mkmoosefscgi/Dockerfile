FROM debian:buster-slim

# Add start script
ADD cgiserver.sh /usr/sbin/cgiserver.sh

RUN apt-get update && apt-get install -y wget gnupg2 python3 \
  && wget -O - http://ppa.moosefs.com/moosefs.key 2>/dev/null | apt-key add - 2>/dev/null \
  && echo "deb http://ppa.moosefs.com/3.0.117/apt/debian/buster buster main" > /etc/apt/sources.list.d/moosefs.list \
  && apt-get update && apt-get install -y moosefs-cgi moosefs-cgiserv moosefs-cli \
  && chown root:root /usr/sbin/cgiserver.sh ; chmod 700 /usr/sbin/cgiserver.sh

# Expose ports master ports
EXPOSE 9425

CMD ["cgiserver.sh"]