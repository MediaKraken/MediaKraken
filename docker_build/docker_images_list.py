"""
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
"""

#ALPINE_MIRROR = 'mksonatype'
ALPINE_MIRROR = 'dl-cdn.alpinelinux.org'

CARGO_CRATES = "crates.io"

#CENTOS_MIRROR = 'mksonatype:8081/repository/repo_yum/'
CENTOS_MIRROR = 'http://mirror.centos.org/'

#DEBIAN_MIRROR = 'mksonatype:8081/repository/repo_apt/'
DEBIAN_MIRROR = 'ftp.us.debian.org'

DOCKER_REPOSITORY = 'mkregistry:5000'
DOCKERHUB_REPOSITORY = 'index.docker.io:443'

# pip3 install --trusted-host mksontatype -i http://mksonatype:8081/repository/pypi/simple -r requirements.txt
# PYPI_MIRROR = 'pypi.python.org'
# PYPI_MIRROR_PORT = ''
PYPI_MIRROR = 'mksontatype'
PYPI_MIRROR_PORT = ':8081'

# PROXY_IP_PORT = '0.0.0.0:8080'
# PROXY_USER_NAME = None
# PROXY_USER_PASS = None

# the data is directory, name of container, base image used to build container

# base OS images to build off of, meaning there is a 'from' in the docker file(s) that use these
STAGE_ONE_IMAGES = {
    'AlpineBase3142Py3': ('mkbase_alpinepy3', 'alpine:3.14.2', 'base'),
    # 'AlpineBase3143Py3': ('mkbase_alpinepy3', 'alpine:3.14.3', 'base'),
    # 'AlpineBase3150Py3': ('mkbase_alpinepy3', 'alpine:3.15.0', 'base'),
    # 'AlpineBase3162Py3': ('mkbase_alpinepy3', 'alpine:3.16.2', 'base'),
    # 'AlpineBase3170Py3': ('mkbase_alpinepy3', 'alpine:3.17.0', 'base'),
    'DebianBase11Py3': ('mkbase_debianpy3', 'python:3.12.0a3-bullseye', 'base'),
}

STAGE_TWO_IMAGES = {}

STAGE_CORE_IMAGES = {
    # broadcast server IP for web and client connectivity, must run from HOST
    'mkbroadcast': ('mkbroadcast', 'scratch', 'core'),

    # chat server via mumble
    'mkchatmumble': ('mkchatmumble', 'alpine:3.14.2', 'core'),

    # chat server via ts3 - free license version
    'mkchatteamspeak': ('mkchatteamspeak', 'alpine:3.14.2', 'core'),

    # process cron jobs from the database to amqp or direct container launch
    'mkcron': ('mkcron', 'busybox:1.35.0-uclibc', 'core'),

    # database via postgresql
    'mkdatabase': ('mkdatabase', 'debian:bullseye-slim', 'core'),

    # download files/etc trailers/etc from ampq records
    'mkdownload': ('mkdownload', 'busybox:1.35.0-uclibc', 'core'),

    # thegamesdb bulk data fetch
    'mkgamesdbnetfetchbulk': ('mkgamesdbnetfetchbulk', 'busybox:1.35.0-uclibc', 'core'),

    # guessit via web rest
    'mkguessitrest': ('mkguessitrest',
                      'tiangolo/uwsgi-nginx-flask:python3.8-alpine-2021-10-02', 'core'),

    # runs control network/ir/if/etc
    'mkhardwarecontrol': ('mkhardwarecontrol', 'busybox:1.35.0-uclibc', 'core'),

    # runs as HOST to find new hardware - run and exit
    'mkhardwarescanner': ('mkhardwarescanner', 'busybox:1.35.0-uclibc', 'core'),

    # inotify of file system changes to amqp
    'mkinotify': ('mkinotify', 'busybox:1.35.0-uclibc', 'core'),

    # download libretro cores that are newer - run and exit
    'mklibretrocorefetchupdate': ('mklibretrocorefetchupdate', 'scratch', 'core'),

    # scan media directories for new media - run and exit
    'mkmediascanner': ('mkmediascanner', 'scratch', 'core'),

    # process metadata for media
    'mkmetadata': ('mkmetadata', 'busybox:1.35.0-uclibc', 'core'),

    # process metadata for mame and other game xml
    'mkmetadatamame': ('mkmetadatamame', 'busybox:1.35.0-uclibc', 'core'),

    # "broadcast" multicast for discovery
    'mkmulticast': ('mkmulticast', 'busybox:1.35.0-uclibc', 'core'),

    # nginx proxy for http to https and some bot blocking
    'mknginx': ('mknginx', 'alpine:3.13', 'core'),

    # database connection pooler
    'mkpgbouncer': ('mkpgbouncer', 'alpine:3.14.3', 'core'),

    # consume and process ampq records
    'mkrabbitconsume': ('mkrabbitconsume', 'busybox:1.35.0-uclibc', 'core'),

    # amqp service (rabbitmq)
    'mkrabbitmq': ('mkrabbitmq', 'alpine:3.11', 'core'),

    # schedulesdirect update
    'mkschedulesdirectupdate': ('mkschedulesdirectupdate', 'busybox:1.35.0-uclibc', 'core'),

    # server for "fat" clients to talk too (local server)
    'mktcpserver': ('mktcpserver', 'busybox:1.35.0-uclibc', 'core'),

    # download tmdb dump of ids in database and insert into downloads - run and exit
    'mktmdbnetfetchbulk': ('mktmdbnetfetchbulk', 'scratch', 'core'),

    # download tmdb dump of ids that were updated - run and exit
    'mktmdbnetfetchupdate': ('mktmdbnetfetchupdate', 'scratch', 'core'),

    # transcode/STREAM media to client - run and exit
    'mktranscode': ('mktranscode', 'busybox:1.35.0-uclibc', 'core'),

    # transmission server
    'mktransmission': ('mktransmission', 'alpine:3.14.2', 'core'),

    # tvheadend
    'mktvheadend': ('mktvheadend', 'alpine:3.12', 'core'),

    # website via rust and rocket
    'mkwebapp': ('mkwebapp', 'busybox:1.35.0-uclibc', 'core'),

    # website for nut
    'mkwebnut': ('mkwebnut', 'ubuntu:18.04', 'core'),
}

STAGE_ONE_GAME_SERVERS = {
    # for hosting games via dosbox and web
    'mkgamebasedosboxweb': ('mkgamebasedosboxweb', 'ubuntu:22.10', 'game_base'),

    # for hosting games via retroarch and web
    'mkgamebaseretroarchweb': ('mkgamebaseretroarchweb', 'debian:buster-slim', 'game_base'),

    # for hosting games via steamcmd
    'mkgamebasesteamcmd': ('mkgamebasesteamcmd', 'debian:10.9-slim', 'game_base'),

    # for hosting games via steamcmd as root
    'mkgamebasesteamcmdroot': ('mkgamebasesteamcmdroot', 'debian:10.9-slim', 'game_base'),

    # for hosting software via wine
    'mkgamebasewine': ('mkgamebasewine', 'debian:10.9-slim', 'game_base'),
}

STAGE_TWO_GAME_SERVERS = {
    # Dockerized battlefield 1942
    'mkgamebf42': ('mkgamebf42', 'ubuntu:14.04', 'game_server'),

    # Factorio Server in docker
    'mkgamefactorio': ('mkgamefactorio', 'ubuntu:14.04', 'game_server'),

    # FAKE Server in docker
    'mkgamekerbalspaceprogram': ('mkgamekerbalspaceprogram', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameminecraft': ('mkgameminecraft', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameq3a': ('mkgameq3a', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameq3a_cpma': ('mkgameq3a_cpma', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameq3a_osp': ('mkgameq3a_osp', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameq3a_rq3': ('mkgameq3a_rq3', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamequake2': ('mkgamequake2', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamequake4': ('mkgamequake4', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamequakelive': ('mkgamequakelive', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_arma3': ('mkgamesteamcmd_arma3', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_csgo': ('mkgamesteamcmd_csgo', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_doubleaction': ('mkgamesteamcmd_doubleaction', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_fistfuloffrags': ('mkgamesteamcmd_fistfuloffrags', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_holdfastnaw': ('mkgamesteamcmd_holdfastnaw', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_insurgency': ('mkgamesteamcmd_insurgency', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_mordhau': ('mkgamesteamcmd_mordhau', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_squad': ('mkgamesteamcmd_squad', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_tf2': ('mkgamesteamcmd_tf2', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamesteamcmd_valheim': ('mkgamesteamcmd_valheim', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameut99': ('mkgameut99', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgameut2004': ('mkgameut2004', 'FAKE', 'game_server'),

    # FAKE Server in docker
    'mkgamewindward': ('mkgamewindward', 'FAKE', 'game_server'),
}

STAGE_ONE_SECURITY_TOOLS = {

}

STAGE_TWO_SECURITY_TOOLS = {

}

STAGE_ONE_TESTING_TOOLS = {
    'mkelk': ('mkelk', 'phusion/baseimage-focal-1.1.0', 'test'),
    'mkftpserver': ('mkftpserver', 'alpine:3.14.2', 'test'),
    'mkjenkins': ('mkjenkins', 'jenkins/jenkins:lts', 'test'),
    'mkselenium': ('mkselenium', 'mkbase_alpinepy3', 'test'),
    'mksonatype': ('mksonatype', 'sonatype/nexus3', 'test'),
    'mktrac': ('mktrac', 'debian:buster-slim', 'test'),
}

STAGE_TWO_TESTING_TOOLS = {

}
