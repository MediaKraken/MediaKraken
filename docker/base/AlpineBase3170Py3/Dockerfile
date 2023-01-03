# Download base image
FROM alpine:3.17.0

LABEL author="Quinn D Granfor, spootdev@gmail.com"
LABEL description="This holds the base image"

ARG ALPMIRROR

# create work dir
RUN mkdir /mediakraken

# copy PIP requirements
COPY requirements.txt /mediakraken

WORKDIR /mediakraken

# pip3 is installed now in community, hence the first sed
RUN sed -i "2 s/^#//" /etc/apk/repositories \
  && sed -i "s/dl-cdn.alpinelinux.org/${ALPMIRROR}/" /etc/apk/repositories \
  && apk add --no-cache \
  busybox==1.35.0-r29 \
  python3==3.10.9-r1 \
  py3-pip==22.3.1-r1 \
  && pip3 install --no-cache-dir --trusted-host mksonatype -i http://mksonatype:8081/repository/pypi/simple -r requirements.txt \
  && rm requirements.txt \
  && adduser -DHs /sbin/nologin metaman

# Do NOT do the below as other containers won't have rights to install/build.
# USER metaman

CMD ["/bin/ash"]
