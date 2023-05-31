#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# can't use links.....as docker build won't have access to linked file

# \cp     is to unalias it

# mkbroadcast
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkbroadcast/src/.

# mkcron
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkcron/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkcron/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkcron/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkcron/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkcron/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkcron/src/.

# mkdownload
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkdownload/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkdownload/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkdownload/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkdownload/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkdownload/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkdownload/src/.

# mkgamesdbnetfetchbulk
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkgamesdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkgamesdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkgamesdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkgamesdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkgamesdbnetfetchbulk/src/database/.

# mkhardwarecontrol
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarecontrol/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarecontrol/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_serial.rs ../docker/core/mkhardwarecontrol/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_telnet.rs ../docker/core/mkhardwarecontrol/src/.

# mkhardwarescanner
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkhardwarescanner/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkhardwarescanner/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_dlna.rs ../docker/core/mkhardwarescanner/src/.
\cp -f --remove-destination ../src/mk_lib_hardware/src/mk_lib_hardware_chromecast.rs ../docker/core/mkhardwarescanner/src/.
\cp -f --remove-destination ../src/mk_lib_hardware/src/mk_lib_hardware_phue.rs ../docker/core/mkhardwarescanner/src/.

# mkinotify
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkinotify/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkinotify/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkinotify/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkinotify/src/database/.
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
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_movie.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_tv.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmediascanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmediascanner/src/database/.

# mkmetadata
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmetadata/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmetadata/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmetadata/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmetadata/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/*.rs ../docker/core/mkmetadata/src/database/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkmetadata/src/.
\cp -f --remove-destination ../src/mk_lib_metadata/src/*.rs ../docker/core/mkmetadata/src/metadata/.
\cp -f --remove-destination ../src/mk_lib_hash/src/mk_lib_hash_sha1.rs ../docker/core/mkmetadata/src/.
\cp -fR --remove-destination ../src/mk_lib_metadata/src/provider ../docker/core/mkmetadata/src/metadata/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkmetadata/src/database/.

# mkmetadatamame
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkmetadatamame/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkmetadatamame/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkmetadatamame/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkmetadatamame/src/database/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkmetadatamame/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game.rs ../docker/core/mkmetadatamame/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game_system.rs ../docker/core/mkmetadatamame/src/database/.

# mkmulticast
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkmulticast/src/.

# mkopenlibrarynetfetchbulk
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mkopenlibrarynetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mkopenlibrarynetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkopenlibrarynetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkopenlibrarynetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkopenlibrarynetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkopenlibrarynetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_book.rs ../docker/core/mkopenlibrarynetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkopenlibrarynetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkopenlibrarynetfetchbulk/src/.

# mkrabbitconsume
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkrabbitconsume/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkrabbitconsume/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkrabbitconsume/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkrabbitconsume/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkrabbitconsume/src/database/.

# mkschedulesdirectupdate
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkschedulesdirectupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkschedulesdirectupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkschedulesdirectupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkschedulesdirectupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkschedulesdirectupdate/src/database/.

# mksharescanner
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mksharescanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_network_share.rs ../docker/core/mksharescanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mksharescanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mksharescanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mksharescanner/src/database/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mksharescanner/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mksharescanner/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mksharescanner/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_nmap.rs ../docker/core/mksharescanner/src/.

# mktcpserver
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktcpserver/src/.

# mktmdbnetfetchbulk
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mktmdbnetfetchbulk/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchbulk/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchbulk/src/.

# mktmdbnetfetchupdate
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_person.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktmdbnetfetchupdate/src/database/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchupdate/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mktmdbnetfetchupdate/src/database/.

# mktranscode
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_ffmpeg.rs ../docker/core/mktranscode/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktranscode/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktranscode/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mktranscode/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mktranscode/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mktranscode/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mktranscode/src/database/.

# mkwebrocket
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_docker.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_pagination.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_game_servers.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_network_share.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_postgresql.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_search.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_sync.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_activity.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_queue.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_profile.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_adult.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_anime.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_book.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game_system.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_game_servers.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_home_media.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_images.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_iradio.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_movie.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music_video.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_remote.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_sports.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_tv.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_adult.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_anime.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_book.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_collection.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game_system.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_image.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music_video.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_person.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_review.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_sports.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv_live.rs ../docker/core/mkwebrocket/src/database/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkwebrocket/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_transmission.rs ../docker/core/mkwebrocket/src/.

# mkwebaxum
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_docker.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_common/src/mk_lib_common_pagination.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_game_servers.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_network_share.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_notification.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_option_status.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_postgresql.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_search.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_sync.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_activity.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_queue.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_user_profile.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_version_schema.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_adult.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_anime.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_book.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_game_system.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/mk_lib_database_game_servers.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_home_media.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_images.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_iradio.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_movie.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_music_video.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_remote.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_sports.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/media/mk_lib_database_media_tv.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_adult.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_anime.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_book.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_collection.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_download_queue.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_game_system.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_image.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_movie.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_music_video.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_person.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_review.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_sports.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_database/src/metadata/mk_lib_database_metadata_tv_live.rs ../docker/core/mkwebaxum/src/database/.
\cp -f --remove-destination ../src/mk_lib_file/src/mk_lib_file.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mkwebaxum/src/.
\cp -f --remove-destination ../src/mk_lib_network/src/mk_lib_network_transmission.rs ../docker/core/mkwebaxum/src/.
