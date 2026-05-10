#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# can't use links.....as docker build won't have access to linked file

# copy the lock file so everything builds the same in docker as well

# mkcron
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkcron/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkcron/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkcron/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkcron/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkcron/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkcron/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkcron/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkcron/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkcron/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkcron/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkcron/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkcron/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkdownload
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkdownload/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkdownload/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkdownload/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkdownload/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkdownload/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkdownload/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkdownload/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkdownload/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkdownload/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkdownload/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkdownload/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkdownload/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkfanotify
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkfanotify/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkfanotify/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkfanotify/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkfanotify/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkfanotify/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkfanotify/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkfanotify/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkfanotify/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkfanotify/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkfanotify/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkfanotify/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkfanotify/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkgamesdbnetfetchbulk
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkgamesdbnetfetchbulk/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkgamesdbnetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkglobalcache
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkglobalcache/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkglobalcache/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkglobalcache/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkglobalcache/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkglobalcache/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkglobalcache/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkglobalcache/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkglobalcache/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkglobalcache/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkglobalcache/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkglobalcache/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkglobalcache/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkhardwarecontrol
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkhardwarecontrol/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkhardwarecontrol/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkhardwarescanner
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkhardwarescanner/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkhardwarescanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkinit
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkinit/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkinit/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkinit/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkinit/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkinit/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkinit/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkinit/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkinit/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkinit/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkinit/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkinit/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkinit/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkinotify
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkinotify/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkinotify/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkinotify/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkinotify/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkinotify/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkinotify/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkinotify/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkinotify/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkinotify/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkinotify/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkinotify/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkinotify/alpine-minirootfs-x86_64.tar.gz --exclude target

# mklibretrocorefetchupdate
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mklibretrocorefetchupdate/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mklibretrocorefetchupdate/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmediascanner
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkmediascanner/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkmediascanner/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkmediascanner/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkmediascanner/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkmediascanner/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkmediascanner/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkmediascanner/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkmediascanner/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkmediascanner/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkmediascanner/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkmediascanner/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_share/Cargo.toml ../docker/core/mkmediascanner/mk_lib_share/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkmediascanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmetadata
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkmetadata/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkmetadata/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkmetadata/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkmetadata/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkmetadata/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkmetadata/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkmetadata/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkmetadata/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkmetadata/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkmetadata/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkmetadata/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkmetadata/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmetadatamame
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkmetadatamame/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkmetadatamame/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmulticast
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkmulticast/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkmulticast/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkmulticast/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkmulticast/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkmulticast/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkmulticast/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkmulticast/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkmulticast/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkmulticast/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkmulticast/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkmulticast/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkmulticast/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmusicbrainz
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkmusicbrainz/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkmusicbrainz/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkopenlibrarynetfetchbulk
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkopenlibrarynetfetchbulk/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkopenlibrarynetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkrabbitconsume
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkrabbitconsume/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkrabbitconsume/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkschedulesdirectupdate
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkschedulesdirectupdate/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkschedulesdirectupdate/alpine-minirootfs-x86_64.tar.gz --exclude target

# mksharescanner
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mksharescanner/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mksharescanner/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mksharescanner/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mksharescanner/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mksharescanner/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mksharescanner/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mksharescanner/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mksharescanner/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mksharescanner/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mksharescanner/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mksharescanner/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mksharescanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mktmdbnetfetchbulk
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mktmdbnetfetchbulk/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mktmdbnetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mktranscode
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mktranscode/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mktranscode/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mktranscode/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mktranscode/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mktranscode/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mktranscode/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mktranscode/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mktranscode/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mktranscode/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mktranscode/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mktranscode/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_share/Cargo.toml ../docker/core/mktranscode/mk_lib_share/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mktranscode/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkwebaxum
\rsync -a ../src/mk_lib_common/Cargo.toml ../docker/core/mkwebaxum/mk_lib_common/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_compression/Cargo.toml ../docker/core/mkwebaxum/mk_lib_compression/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_database/Cargo.toml ../docker/core/mkwebaxum/mk_lib_database/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_file/Cargo.toml ../docker/core/mkwebaxum/mk_lib_file/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hardware/Cargo.toml ../docker/core/mkwebaxum/mk_lib_hardware/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_hash/Cargo.toml ../docker/core/mkwebaxum/mk_lib_hash/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_image/Cargo.toml ../docker/core/mkwebaxum/mk_lib_image/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_logging/Cargo.toml ../docker/core/mkwebaxum/mk_lib_logging/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_metadata/Cargo.toml ../docker/core/mkwebaxum/mk_lib_metadata/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_network/Cargo.toml ../docker/core/mkwebaxum/mk_lib_network/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_rabbitmq/Cargo.toml ../docker/core/mkwebaxum/mk_lib_rabbitmq/Cargo.toml --exclude target
\rsync -a ../src/mk_lib_share/Cargo.toml ../docker/core/mkwebaxum/mk_lib_share/Cargo.toml --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkwebaxum/alpine-minirootfs-x86_64.tar.gz --exclude target
