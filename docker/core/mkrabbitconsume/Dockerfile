FROM mediakraken/mkbase_rust_debian:dev as cargo-build

ARG BRANCHTAG

WORKDIR /mediakraken

COPY ./ .

RUN apt update && apt install libudev-dev -y
RUN if [ "${BRANCHTAG}" = "prod" ]; \
  then cargo build --release \
  && cp /mediakraken/target/release/mk* /mediakraken/target/.; \
  else cargo build \
  && cp /mediakraken/target/debug/mk* /mediakraken/target/.; \
  fi

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-20230522-slim AS runtime
RUN apt-get update && apt-get -y install wget \
  && wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait &&\
  chmod +x /wait \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/bash /sh

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkrabbitconsume /mkrabbitconsume

# Use an unprivileged user.
# USER mediakraken:mediakraken
