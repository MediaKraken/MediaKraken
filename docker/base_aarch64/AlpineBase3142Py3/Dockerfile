# Download base image
FROM alpine:3.14.2

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the base image"

ARG ALPMIRROR
ARG PIPMIRROR

# create work dir
RUN mkdir /mediakraken

# copy PIP requirements
COPY requirements.txt /mediakraken

WORKDIR /mediakraken

# pip3 is installed now in community, hence the first sed
RUN sed -i "2 s/^#//" /etc/apk/repositories \
  && sed -i "s/dl-cdn.alpinelinux.org/${ALPMIRROR}/" /etc/apk/repositories \
  && apk add --no-cache \
  busybox==1.33.1-r3 \
  python3==3.9.5-r1 \
  py3-pip==20.3.4-r1 \
  && pip3 install --no-cache-dir --upgrade pip==21.3.1 \
  && pip3 install --no-cache-dir --trusted-host ${PIPMIRROR} -i https://${PIPMIRROR}/simple -r requirements.txt \
  && rm requirements.txt \
  && adduser -DHs /sbin/nologin metaman

# Do NOT do the below as other containers won't have rights to install/build.
# USER metaman

CMD ["/bin/ash"]
