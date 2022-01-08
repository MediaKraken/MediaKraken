#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# mkcron
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_cron.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkcron/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkcron/src/.

# mkdownload
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkdownload/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkdownload/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkdownload/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkdownload/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkdownload/src/.

# mkhardwarecontrol
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkhardwarecontrol/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkhardwarecontrol/src/.

# mkhardwarescanner
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkhardwarescanner/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkhardwarescanner/src/.

# mklibretrocorefetchupdate
cp src/mk_lib_compression/src/mk_lib_compression.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_hash/src/mk_lib_hash_md5.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mklibretrocorefetchupdate/src/.

# mkmediascanner
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_common/src/mk_lib_common_media_extension.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_file/src/mk_lib_file.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_library.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/media/mk_lib_database_media.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_notification.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmediascanner/src/.

# mkmetadata
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmetadata/src/.

# mkmetadatamame
cp src/mk_lib_compression/src/mk_lib_compression.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkmetadatamame/src/.

# mknotify
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_library.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkinotify/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkinotify/src/.

# mkrabbitconsume
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkrabbitconsume/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkrabbitconsume/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkrabbitconsume/src/.

# mkschedulesdirectupdate
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkschedulesdirectupdate /src/.

# mktmdbnetfetchbulk
cp src/mk_lib_common/src/mk_lib_common.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_compression/src/mk_lib_compression.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchbulk/src/.

# mktmdbnetfetchupdate
cp src/mk_lib_common/src/mk_lib_common.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_compression/src/mk_lib_compression.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_network/src/mk_lib_network.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktmdbnetfetchupdate/src/.

# mktranscode
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mktranscode/src/.

# mkwebapp
cp src/mk_lib_database/src/mk_lib_database.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkwebapp/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkwebapp/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkwebapp/src/.
cp src/mk_lib_file/src/mk_lib_file.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkwebapp/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs /var/lib/go-agent/pipelines/mediakraken_core_pipeline/docker/core/mkwebapp/src/.