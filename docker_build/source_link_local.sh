#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# mkcron
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkcron/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkcron/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkcron/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkcron/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkcron/src/.

# mkdownload
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkdownload/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkdownload/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkdownload/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkdownload/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkdownload/src/.

# mkhardwarecontrol
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarecontrol/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarecontrol/src/.

# mkhardwarescanner
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarescanner/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarescanner/src/.

# mklibretrocorefetchupdate
ln -s ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mklibretrocorefetchupdate/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mklibretrocorefetchupdate/src/.
ln -s ../src/mk_lib_hash/src/mk_lib_hash_md5.rs ../docker/core/mklibretrocorefetchupdate/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mklibretrocorefetchupdate/src/.

# mkmediascanner
ln -s ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_common/src/mk_lib_common_media_extension.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmediascanner/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmediascanner/src/.

# mkmetadata
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmetadata/src/.

# mkmetadatamame
ln -s ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mkmetadatamame/src/.

# mknotify
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkinotify/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkinotify/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkinotify/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkinotify/src/.

# mkrabbitconsume
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkrabbitconsume/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkrabbitconsume/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkrabbitconsume/src/.

# mkschedulesdirectupdate
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkschedulesdirectupdate /src/.

# mktmdbnetfetchbulk
ln -s ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchbulk/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchbulk/src/.

# mktmdbnetfetchupdate
ln -s ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchupdate/src/.
ln -s ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchupdate/src/.

# mktranscode
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktranscode/src/.

# mkwebapp
ln -s ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkwebapp/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkwebapp/src/.
ln -s ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkwebapp/src/.
ln -s ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkwebapp/src/.
ln -s ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkwebapp/src/.
