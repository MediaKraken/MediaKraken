FROM jenkins/jenkins:lts
USER root
RUN apt-get update && apt-get install -y python3-pip sloccount libpq-dev libcurl4-openssl-dev \
  mesa-common-dev libssl-dev libusb-dev
ADD requirements.txt /
RUN pip3 install -r /requirements.txt && rm /requirements.txt
USER jenkins
