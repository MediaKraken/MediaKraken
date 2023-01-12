FROM ubuntu:18.04

LABEL maintainer="Jakezp <jakezp@gmail.com>"

ENV DEBIAN_FRONTEND noninteractive
ENV NUT_VERSION 2.7.4

ENV UPS_NAME="ups"
ENV UPS_DESC="UPS"
ENV UPS_DRIVER="blazer_usb"
ENV UPS_PORT="auto"

ENV API_USER="monuser"
ENV API_PASSWORD=""
ENV ADMIN_PASSWORD=""

ENV SHUTDOWN_CMD="echo 'System shutdown not configured!'"

# Update and install packages
RUN apt-get update \
    && apt-get install nut curl git python3-pip python-pip tzdata -yq \
    && rm -rf /var/lib/apt/lists/*

# Install and configure webnut
RUN pip install setuptools \
    && pip install config \
    && mkdir /app\
    && cd /app \
    && git clone https://github.com/rshipp/python-nut2.git \
    && cd python-nut2 \
    && python setup.py install \
    && cd .. \
    && git clone https://github.com/rshipp/webNUT.git \
    && cd webNUT  \
    && pip install -e . 

# Add run and set permissions
COPY run.sh /run.sh
RUN chmod +x /run.sh

WORKDIR /app/webNUT

VOLUME ["/app/webNUT/webnut/", "/etc/nut/"]

EXPOSE 3493 6543

CMD ["/run.sh"]
