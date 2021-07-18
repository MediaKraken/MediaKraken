#!/bin/sh

# mkcron
cp ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_database/src/mk_lib_database_cron.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkinotify/src/.

# mknotify
cp ../src/mk_lib_database/src/mk_lib_database.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_database/src/mk_lib_database_library.rs ../docker/core/mkinotify/src/.
cp ../src/mk_lib_logging/src/mk_lib_logging.rs ../docker/core/mkinotify/src/.
