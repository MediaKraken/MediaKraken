FROM rust:1.56.0 as cargo-build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

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

WORKDIR /mediakraken

COPY ./ .
COPY ../../../src/mk_lib_common/src/mk_lib_common.rs .
COPY ../../../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs .
COPY ../../../src/mk_lib_compression/src/mk_lib_compression.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_metadata.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_metadata_download_queue.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_version.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_version_schema.rs .
COPY ../../../src/mk_lib_logging/src/mk_lib_logging.rs .
COPY ../../../src/mk_lib_network/src/mk_lib_network.rs .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
#FROM busybox:1.33.1-uclibc
FROM scratch

# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group

## copy the files
#COPY --from=cargo-build /mediakraken/movie.gz /mediakraken/movie.gz
#COPY --from=cargo-build /mediakraken/tv.gz /mediakraken/tv.gz

WORKDIR /mediakraken

# Copy our build
COPY --from=cargo-build /mediakraken/target/x86_64-unknown-linux-musl/release/mktmdbnetfetchbulk ./

# Use an unprivileged user.
USER mediakraken:mediakraken

CMD ["/mediakraken/mktmdbnetfetchbulk"]
