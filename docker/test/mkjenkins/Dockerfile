FROM jenkins/jenkins:lts
USER root
RUN apt-get update && apt-get install -y apt-utils python3-pip sloccount cloc \
  libpq-dev libcurl4-openssl-dev apache2-utils wapiti \
  mesa-common-dev libssl-dev libusb-dev cppcheck
ADD requirements.txt /
RUN pip3 install -r /requirements.txt && rm /requirements.txt
COPY --chown=jenkins:jenkins plugins.txt /usr/share/jenkins/ref/plugins.txt
RUN /usr/local/bin/install-plugins.sh < /usr/share/jenkins/ref/plugins.txt
USER jenkins
