#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# can't use links.....as docker build won't have access to linked file

# copy the lock file so everything builds the same in docker as well

# mkbroadcast
\rsync -a ../src/mk_lib_* ../docker/core/mkbroadcast/. --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkbroadcast/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkcron
\rsync -a ../src/mk_lib_* ../docker/core/mkcron/. --exclude target

# mkdownload
\rsync -a ../src/mk_lib_* ../docker/core/mkdownload/. --exclude target

# mkgamesdbnetfetchbulk
\rsync -a ../src/mk_lib_* ../docker/core/mkgamesdbnetfetchbulk/. --exclude target

# mkglobalcache
\rsync -a ../src/mk_lib_* ../docker/core/mkglobalcache/. --exclude target

# mkhardwarecontrol
\rsync -a ../src/mk_lib_* ../docker/core/mkhardwarecontrol/. --exclude target

# mkhardwarescanner
\rsync -a ../src/mk_lib_* ../docker/core/mkhardwarescanner/. --exclude target

# mkinotify
\rsync -a ../src/mk_lib_* ../docker/core/mkinotify/. --exclude target

# mklibretrocorefetchupdate
\rsync -a ../src/mk_lib_* ../docker/core/mklibretrocorefetchupdate/. --exclude target

# mkmediascanner
\rsync -a ../src/mk_lib_* ../docker/core/mkmediascanner/. --exclude target

# mkmetadata
\rsync -a ../src/mk_lib_* ../docker/core/mkmetadata/. --exclude target

# mkmetadatamame
\rsync -a ../src/mk_lib_* ../docker/core/mkmetadatamame/. --exclude target

# mkmulticast
\rsync -a ../src/mk_lib_* ../docker/core/mkmulticast/. --exclude target

# mkmusicbrainz
\rsync -a ../src/mk_lib_* ../docker/core/mkmusicbrainz/. --exclude target

# mkopenlibrarynetfetchbulk
\rsync -a ../src/mk_lib_* ../docker/core/mkopenlibrarynetfetchbulk/. --exclude target

# mkrabbitconsume
\rsync -a ../src/mk_lib_* ../docker/core/mkrabbitconsume/. --exclude target

# mkschedulesdirectupdate
\rsync -a ../src/mk_lib_* ../docker/core/mkschedulesdirectupdate/. --exclude target

# mksharescanner
\rsync -a ../src/mk_lib_* ../docker/core/mksharescanner/. --exclude target

# mktmdbnetfetchbulk
\rsync -a ../src/mk_lib_* ../docker/core/mktmdbnetfetchbulk/. --exclude target

# mktmdbnetfetchupdate
\rsync -a ../src/mk_lib_* ../docker/core/mktmdbnetfetchupdate/. --exclude target

# mktranscode
\rsync -a ../src/mk_lib_* ../docker/core/mktranscode/. --exclude target

# mkwebaxum
\rsync -a ../src/mk_lib_* ../docker/core/mkwebaxum/. --exclude target
