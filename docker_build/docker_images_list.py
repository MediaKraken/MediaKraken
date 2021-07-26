'''
  Copyright (C) 2021 Quinn D Granfor <spootdev@gmail.com>

  This program is free software; you can redistribute it and/or
  modify it under the terms of the GNU General Public License
  version 2, as published by the Free Software Foundation.

  This program is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  General Public License version 2 for more details.

  You should have received a copy of the GNU General Public License
  version 2 along with this program; if not, write to the Free
  Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
  MA 02110-1301, USA.
'''

# ALPINE_MIRROR = 'th-alpinemirror-1.beaverbay.local'
ALPINE_MIRROR = 'dl-cdn.alpinelinux.org'

# DEBIAN_MIRROR = 'th-debianmirror-1.beaverbay.local'
DEBIAN_MIRROR = 'ftp.us.debian.org'

# PYPI_MIRROR = 'th-bandersnatch-1'
PYPI_MIRROR = 'pypi.python.org'

DOCKER_REPOSITORY = 'th-registry-1.beaverbay.local:5000'
DOCKERHUB_REPOSITORY = 'index.docker.io:443'

PROXY_IP_PORT = '0.0.0.0:8080'
PROXY_USER_NAME = None
PROXY_USER_PASS = None

# the data is directory, name of container, base image used to build container

# base OS images to build off of, meaning there is a 'from' in the docker file(s) that use these
# or simply stand alone images
STAGE_ONE_IMAGES = {
    'AlpineBase3135Py3': ('mkbase_alpinepy3', 'alpine:3.13.5', 'base'),
}

STAGE_TWO_IMAGES = {}

STAGE_ONE_GAME_SERVERS = {

}

STAGE_TWO_GAME_SERVERS = {}

STAGE_CORE_IMAGES = {
    # broadcast server IP for web and client connectivity, must run from HOST
    'mkbroadcast': ('mkbroadcast', 'scratch', 'core'),

    # chat server via mumble
    'mkchatmumble': ('mkchatmumble', 'alpine:3.13.5', 'core'),

    # chat server via ts3 - free license version
    'mkchatteamspeak': ('mkchatteamspeak', 'alpine:3.13.5', 'core'),

    # process cron jobs from the database to amqp or direct container launch
    'mkcron': ('mkcron', 'busybox:1.33.1-uclibc', 'core'),

    # database via postgresql
    'mkdatabase': ('mkdatabase', 'debian:buster-slim', 'core'),

    # download files/etc trailers/etc from ampq records
    'mkdownload': ('mkdownload', 'busybox:1.33.1-uclibc', 'core'),

    # runs control network/ir/if/etc
    'mkhardwarecontrol': ('mkhardwarecontrol', 'busybox:1.33.1-uclibc', 'core'),

    # runs as HOST to find new hardware - run and exit
    'mkhardwarescanner': ('mkhardwarescanner', 'busybox:1.33.1-uclibc', 'core'),

    # inotify of file system changes to amqp
    'mkinotify': ('mkinotify', 'busybox:1.33.1-uclibc', 'core'),

    # download libretro cores that are newer - run and exit
    'mklibretrocorefetchupdate': ('mklibretrocorefetchupdate', 'scratch', 'core'),

    # scan media directories for new media - run and exit
    'mkmediascanner': ('mkmediascanner', 'scratch', 'core'),

    # nginx proxy for http to https and some bot blocking
    'mknginx': ('mknginx', 'alpine:3.10', 'core'),

    # database connection pooler
    'mkpgbouncer': ('mkpgbouncer', 'alpine:3.13.5', 'core'),

    # amqp service (rabbitmq)
    'mkrabbitmq': ('mkrabbitmq', 'alpine:3.11', 'core'),

    # download files/etc trailers/etc from ampq records
    'mkrabbitconsume': ('mkrabbitconsume', 'busybox:1.33.1-uclibc', 'core'),

    # download tmdb dump of ids in database and insert into downloads - run and exit
    'mktmdbnetfetchbulk': ('mktmdbnetfetchbulk', 'scratch', 'core'),

    # download tmdb dump of ids that were updated - run and exit
    'mktmdbnetfetchupdate': ('mktmdbnetfetchupdate', 'scratch', 'core'),

    # transcode/STREAM media to client - run and exit
    'mktranscode': ('mktranscode', 'alpine:3.13.5', 'core'),

    # transmission server
    'mktransmission': ('mktransmission', 'alpine:3.13.5', 'core'),

    # website via rust and actixweb
    'mkwebapp': ('mkwebapp', 'mkbase_alpinepy3', 'core'),  # TODO
}

STAGE_ONE_SECURITY_TOOLS = {

}

STAGE_TWO_SECURITY_TOOLS = {

}

STAGE_ONE_TESTING_TOOLS = {}

STAGE_TWO_TESTING_TOOLS = {}
