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
COPY ../../../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs .
COPY ../../../src/mk_lib_common/src/mk_lib_common_media_extension.rs .
COPY ../../../src/mk_lib_file/src/mk_lib_file.rs .
COPY ../../../src/mk_lib_logging/src/mk_lib_logging.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_library.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_media.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_metadata_download_queue.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_notification.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_version.rs .
COPY ../../../src/mk_lib_database/src/mk_lib_database_version_schema.rs .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=cargo-build /etc/passwd /etc/passwd
COPY --from=cargo-build /etc/group /etc/group

WORKDIR /mediakraken

# Copy our build
COPY --from=cargo-build /mediakraken/target/x86_64-unknown-linux-musl/release/mkmediascanner ./

# Use an unprivileged user.
USER mediakraken:mediakraken
