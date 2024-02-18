#!/bin/sh

# Don't think you can get rid of this.
# Docker build is dumb and can only access stuff in root directory

# can't use links.....as docker build won't have access to linked file

# copy the lock file so everything builds the same in docker as well

# mkbroadcast
#\rsync -a ../src/mk_lib_* ../docker/core/mkbroadcast/. --exclude target
\rsync -a alpine-minirootfs* ../docker/core/mkbroadcast/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkcron
\rsync -a alpine-minirootfs* ../docker/core/mkcron/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkdownload
\rsync -a alpine-minirootfs* ../docker/core/mkdownload/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkgamesdbnetfetchbulk
\rsync -a alpine-minirootfs* ../docker/core/mkgamesdbnetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkglobalcache
\rsync -a alpine-minirootfs* ../docker/core/mkglobalcache/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkhardwarecontrol
\rsync -a alpine-minirootfs* ../docker/core/mkhardwarecontrol/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkhardwarescanner
\rsync -a alpine-minirootfs* ../docker/core/mkhardwarescanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkinotify
\rsync -a alpine-minirootfs* ../docker/core/mkinotify/alpine-minirootfs-x86_64.tar.gz --exclude target

# mklibretrocorefetchupdate
\rsync -a alpine-minirootfs* ../docker/core/mklibretrocorefetchupdate/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmediascanner
\rsync -a alpine-minirootfs* ../docker/core/mkmediascanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmetadata
\rsync -a alpine-minirootfs* ../docker/core/mkmetadata/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmetadatamame
\rsync -a alpine-minirootfs* ../docker/core/mkmetadatamame/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmulticast
\rsync -a alpine-minirootfs* ../docker/core/mkmulticast/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkmusicbrainz
\rsync -a alpine-minirootfs* ../docker/core/mkmusicbrainz/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkopenlibrarynetfetchbulk
\rsync -a alpine-minirootfs* ../docker/core/mkopenlibrarynetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkrabbitconsume
\rsync -a alpine-minirootfs* ../docker/core/mkrabbitconsume/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkschedulesdirectupdate
\rsync -a alpine-minirootfs* ../docker/core/mkschedulesdirectupdate/alpine-minirootfs-x86_64.tar.gz --exclude target

# mksharescanner
\rsync -a alpine-minirootfs* ../docker/core/mksharescanner/alpine-minirootfs-x86_64.tar.gz --exclude target

# mktmdbnetfetchbulk
\rsync -a alpine-minirootfs* ../docker/core/mktmdbnetfetchbulk/alpine-minirootfs-x86_64.tar.gz --exclude target

# mktmdbnetfetchupdate
\rsync -a alpine-minirootfs* ../docker/core/mktmdbnetfetchupdate/alpine-minirootfs-x86_64.tar.gz --exclude target

# mktranscode
\rsync -a alpine-minirootfs* ../docker/core/mktranscode/alpine-minirootfs-x86_64.tar.gz --exclude target

# mkwebaxum
\rsync -a alpine-minirootfs* ../docker/core/mkwebaxum/alpine-minirootfs-x86_64.tar.gz --exclude target
