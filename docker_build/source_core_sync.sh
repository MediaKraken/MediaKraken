#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# mkcron
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_cron.rs docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mkcron/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mkcron/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkcron/src/.

# mkdownload
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkdownload/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mkdownload/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mkdownload/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkdownload/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mkdownload/src/.

# mkhardwarecontrol
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkhardwarecontrol/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mkhardwarecontrol/src/.

# mkhardwarescanner
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkhardwarescanner/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mkhardwarescanner/src/.

# mklibretrocorefetchupdate
cp src/mk_lib_compression/src/mk_lib_compression.rs docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_hash/src/mk_lib_hash_md5.rs docker/core/mklibretrocorefetchupdate/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mklibretrocorefetchupdate/src/.

# mkmediascanner
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_common/src/mk_lib_common_media_extension.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_file/src/mk_lib_file.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_library.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/media/mk_lib_database_media.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_notification.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mkmediascanner/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mkmediascanner/src/.

# mkmetadata
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkmetadata/src/.

# mkmetadatamame
cp src/mk_lib_compression/src/mk_lib_compression.rs docker/core/mkmetadatamame/src/.

# mknotify
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_library.rs docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mkinotify/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mkinotify/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkinotify/src/.

# mkrabbitconsume
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkrabbitconsume/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkrabbitconsume/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mkrabbitconsume/src/.

# mkschedulesdirectupdate
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkschedulesdirectupdate /src/.

# mktmdbnetfetchbulk
cp src/mk_lib_common/src/mk_lib_common.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_compression/src/mk_lib_compression.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mktmdbnetfetchbulk/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mktmdbnetfetchbulk/src/.

# mktmdbnetfetchupdate
cp src/mk_lib_common/src/mk_lib_common.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_common/src/mk_lib_common_enum_media_type.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_compression/src/mk_lib_compression.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mktmdbnetfetchupdate/src/.
cp src/mk_lib_network/src/mk_lib_network.rs docker/core/mktmdbnetfetchupdate/src/.

# mktranscode
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mktranscode/src/.

# mkwebapp
cp src/mk_lib_database/src/mk_lib_database.rs docker/core/mkwebapp/src/.
cp src/mk_lib_database/src/mk_lib_database_version.rs docker/core/mkwebapp/src/.
cp src/mk_lib_database/src/mk_lib_database_version_schema.rs docker/core/mkwebapp/src/.
cp src/mk_lib_file/src/mk_lib_file.rs docker/core/mkwebapp/src/.
cp src/mk_lib_logging/src/mk_lib_logging.rs docker/core/mkwebapp/src/.