ARG BRANCHTAG

FROM mediakraken/mkbase_rust_debian:"${BRANCHTAG}" as cargo-build

WORKDIR /mediakraken

COPY ./ .

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
WORKDIR /mediakraken
RUN apt-get update && apt-get -y install wget \
  && wget -qO /wait https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait \
  && chmod +x /wait \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Import from builder.
# COPY --from=cargo-build /etc/passwd /etc/passwd
# COPY --from=cargo-build /etc/group /etc/group
COPY --from=cargo-build /bin/bash /sh
COPY --from=cargo-build /mediakraken/templates /mediakraken/templates
COPY --from=cargo-build /mediakraken/static /mediakraken/static
COPY --from=cargo-build /mediakraken/Rocket.toml /mediakraken/Rocket.toml

# Copy our build
COPY --from=cargo-build /mediakraken/target/mkwebapp /mediakraken/mkwebapp

# Use an unprivileged user.
# USER mediakraken:mediakraken
