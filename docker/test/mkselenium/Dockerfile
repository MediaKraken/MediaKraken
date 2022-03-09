# Download base image
FROM th-registry-1.beaverbay.local:5000/mediakraken/mkbase_alpinepy3:refactor

ARG ALPMIRROR
ARG PIPMIRROR

# copy PIP requirements
COPY requirements.txt /mediakraken
WORKDIR /mediakraken

RUN sed -i "s/dl-cdn.alpinelinux.org/${ALPMIRROR}/" /etc/apk/repositories \
  && apk add --no-cache \
  alpine-sdk \
  python3-dev \
  && pip3 install --no-cache-dir --trusted-host ${PIPMIRROR} -i https://${PIPMIRROR}/simple -r requirements.txt

# Copy the source files for the app
COPY src /mediakraken

CMD ["/bin/ash"]
