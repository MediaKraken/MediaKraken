# don't use the slim version as it's just a stage one image
# as too much stuff is missing causing stuff later to not compile
FROM rust:1.69.0-bullseye

RUN apt update && apt install libudev-dev wget -y \
  && wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait \
  && chmod +x /wait
  
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
    "${USER}" \
    && cargo search lazy_static
