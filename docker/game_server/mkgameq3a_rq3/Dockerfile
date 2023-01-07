ARG BRANCHTAG
FROM mediakraken/mkgameq3a:${BRANCHTAG}

LABEL maintainer="jberrenberg v1.0"

USER root
# add rq3 
RUN \
  echo "# INSTALL DEPENDENCIES ##########################################" && \
  wget -qO ftp://ftp.nsysu.edu.tw/BSD/FreeBSD/ports/distfiles/ReactionQuake3-v3.2-Full.zip && \
  su ioq3srv -c "unzip ReactionQuake3-v3.2-Full.zip -d ~/ioquake3" && \
  echo "# CLEAN UP ######################################################" && \
  rm ReactionQuake3-v3.2-Full.zip

USER ioq3srv

ENTRYPOINT ["/home/ioq3srv/ioquake3/ioq3ded.x86_64", "+set", "fs_game", "rq3"]
