#!/bin/sh

# mkcron
cp ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkcron/src/.
cp ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkcron/src/.
cp ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkcron/src/.

# mknotify
cp ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkinotify/src/.

# mktmdbnetfetchbulk
cp ../src/mk_lib_common/src/mk_lib_common.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_compression/src/mk_lib_compression.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_database/src/mk_lib_database_download.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_database/src/mk_lib_database_metadata.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mktmdbnetfetchbulk/src/.
cp ../src/mk_lib_network/src/mk_lib_network.rs ../docker/core/mktmdbnetfetchbulk/src/.