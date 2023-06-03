# don't use the slim version as it's just a stage one image
# as too much stuff is missing causing stuff later to not compile
FROM rust:1.69.0-bullseye

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
