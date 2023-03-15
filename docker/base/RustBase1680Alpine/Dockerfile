FROM rust:1.68.0-alpine

RUN apk update && \
  apk add alpine-sdk sqlite-dev openssl1.1-compat-dev

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
