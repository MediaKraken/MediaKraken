FROM ubuntu:14.04

LABEL author="Chris Grimmett <chris@grimtech.net>"

# add server assets to container
COPY ./assets /srv/assets

# satisfy setup script and bf1942 dependencies
# then setup battlefield server package
#   * downloads needed bf server files
#   * extracts 1.6 files
#   * patches to 1.61

RUN apt-get update && \
  apt-get -y install wget expect libc6-i386 lib32ncurses5 libc6-dev-i386 \
  && bash -x /srv/assets/setup.sh

# set default command for running this container (run bf server)
CMD /srv/start.sh
