#!/bin/sh

set -e

# make sure we never run 2 rsync at the same time
lockfile="/tmp/alpine-mirror.lock"
if [ -z "$flock" ] ; then
  exec env flock=1 flock -n $lockfile $0 "$@"
fi

src=rsync://rsync.alpinelinux.org/alpine/ 
dest=/usr/share/nginx/html

/usr/bin/rsync -prua \
    --exclude-from /etc/rsync/exclude.txt \
    --delete \
    --timeout=600 \
    --delay-updates \
    --delete-after \
    "$src" "$dest"
