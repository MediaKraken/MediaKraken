#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# can't use links.....as docker build won't have access to linked file

# \cp     is to unalias it

# mkcron
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkcron/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkcron/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkcron/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkcron/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkcron/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkcron/src/.

# mkdownload
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkdownload/src/.

# mkgamesdbnetfetchbulk
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkgamesdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkgamesdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkgamesdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkgamesdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkgamesdbnetfetchbulk/src/.

# mkhardwarecontrol
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarecontrol/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarecontrol/src/.

# mkhardwarescanner
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarescanner/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarescanner/src/.

# mkinotify
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkinotify/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkinotify/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkinotify/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkinotify/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkinotify/src/.

# mklibretrocorefetchupdate
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mklibretrocorefetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mklibretrocorefetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_hash/src/mk_lib_hash_md5.rs ../docker/core/mklibretrocorefetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_hash/src/mk_lib_hash_crc32.rs ../docker/core/mklibretrocorefetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mklibretrocorefetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mklibretrocorefetchupdate/src/.

# mkmediascanner
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_media_extension.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_movie.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_tv.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmediascanner/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmediascanner/src/.

# mkmetadata
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_metadata/src/*.rs ../docker/core/mkmetadata/src/metadata/.
\cp -f --remove-destination ../src/mk_lib_hash/src/mk_lib_hash_sha1.rs ../docker/core/mkmetadata/src/.

# mkmetadatamame
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game_system.rs ../docker/core/mkmetadatamame/src/.

# mkrabbitconsume
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkrabbitconsume/src/.

# mkschedulesdirectupdate
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkschedulesdirectupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkschedulesdirectupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkschedulesdirectupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkschedulesdirectupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkschedulesdirectupdate/src/.

# mktmdbnetfetchbulk
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchbulk/src/.

# mktmdbnetfetchupdate
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mktmdbnetfetchupdate/src/.

# mktranscode
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktranscode/src/.

# mkwebapp
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_game_servers.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_postgresql.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_search.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_sync.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_activity.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_queue.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_profile.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_adult.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_anime.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_book.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game_system.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_home_media.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_images.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_iradio.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_movie.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music_video.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_remote.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_sports.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_tv.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_adult.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_anime.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_book.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_collection.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game_system.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_image.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music_video.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_person.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_review.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_sports.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv_live.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkwebapp/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkwebapp/src/.

