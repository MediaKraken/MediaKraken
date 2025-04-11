FROM debian:bookworm-20250407-slim
RUN apt update && apt install ora2pg wget ca-certificates -y \
  && update-ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*