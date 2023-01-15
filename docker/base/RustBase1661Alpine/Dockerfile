FROM rust:1.66.1-alpine

RUN apk update && \
  apk add alpine-sdk sqlite-dev openssl-dev

# Create appuser
ENV USER=mediakraken
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"
