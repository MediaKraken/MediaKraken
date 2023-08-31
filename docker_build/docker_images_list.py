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

# ALPINE_MIRROR = 'mksonatype'
ALPINE_MIRROR = 'dl-cdn.alpinelinux.org'

CARGO_CRATES = "crates.io"

CENTOS_MIRROR = 'mksonatype:8081/repository/repo_yum/'
# CENTOS_MIRROR = 'http://mirror.centos.org/'

DEBIAN_MIRROR = 'mksonatype:8081/repository/repo_apt/'
# DEBIAN_MIRROR = 'ftp.us.debian.org'

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
    # 'AlpineBase3173Py3': ('mkbase_alpinepy3', 'alpine:3.17.3', 'base'),
    'AlpineBaseFFMPEG': ('mkbase_alpine_ffmpeg', 'alpine:3.17.3', 'base'),
    'DebianBaseFFMPEG': ('mkbase_debian_ffmpeg', 'debian:bookworm-20230703-slim', 'base'),
    'DebianBase11Py3': ('mkbase_debianpy3', 'python:3.12.0a3-bullseye', 'base'),
    'RustBaseAlpine': ('mkbase_rust_alpine', 'rust:1.70.0-alpine', 'base'),
    'RustBaseDebian': ('mkbase_rust_debian', 'rust:1.70.0', 'base'),
}

STAGE_TWO_IMAGES = {}

STAGE_CORE_IMAGES = {
    # barman postgresql backup server
    'mkbarman': ('mkbarman', 'debian:buster', 'core'),

    # broadcast server IP for web and client connectivity, must run from HOST
    # 'mkbroadcast': ('mkbroadcast', 'scratch', 'core'),   # retired!  mkmulticast instead

    # chat server via mumble
    'mkchatmumble': ('mkchatmumble', 'alpine:3.14.2', 'core'),

    # chat server via ts3 - free license version
    'mkchatteamspeak': ('mkchatteamspeak', 'alpine:3.14.2', 'core'),

    # process cron jobs from the database to amqp or direct container launch
    'mkcron': ('mkcron', 'scratch', 'core'),

    # database via postgresql
    # 'mkdatabase': ('mkdatabase', 'debian:bullseye-slim', 'core'),

    # database via postgresql/citus
    'mkdatabase_citus': ('mkdatabase', 'alpine 3.18.3', 'core'),

    # download files/etc trailers/etc from ampq records
    'mkdownload': ('mkdownload', 'scratch', 'core'),

    # filebeat
    'mkfilebeat': ('mkfilebeat', 'elastic/filebeat:7.17.10', 'core'),

    # thegamesdb bulk data fetch
    'mkgamesdbnetfetchbulk': ('mkgamesdbnetfetchbulk', 'scratch', 'core'),

    # download manufactuer stuff from global cache
    'mkglobalcache': ('mkglobalcache', 'scratch', 'core'),

    # guessit via web rest
    'mkguessitrest': ('mkguessitrest',
                      'tiangolo/uwsgi-nginx-flask:python3.10-2023-08-28', 'core'),

    # haproxy
    'mkhaproxy': ('mkhaproxy', 'alpine:3.17', 'core'),

    # runs control network/ir/if/etc
    'mkhardwarecontrol': ('mkhardwarecontrol', 'scratch', 'core'),

    # runs as HOST to find new hardware - run and exit
    'mkhardwarescanner': ('mkhardwarescanner', 'scratch', 'core'),

    # inotify of file system changes to amqp
    'mkinotify': ('mkinotify', 'scratch', 'core'),

    # download libretro cores that are newer - run and exit
    'mklibretrocorefetchupdate': ('mklibretrocorefetchupdate', 'scratch', 'core'),

    # scan media directories for new media - run and exit
    'mkmediascanner': ('mkmediascanner', 'scratch', 'core'),

    # process metadata for media
    'mkmetadata': ('mkmetadata', 'scratch', 'core'),

    # process metadata for mame and other game xml
    'mkmetadatamame': ('mkmetadatamame', 'scratch', 'core'),

    # "broadcast" multicast for discovery
    'mkmulticast': ('mkmulticast', 'scratch', 'core'),

    # musicbrainz load
    'mkmusicbrainz': ('mkmusicbrainz', 'scratch', 'core'),

    # nginx proxy for http to https and some bot blocking
    'mknginx': ('mknginx', 'alpine:3.13', 'core'),

    # nginx pagespeed - retired by apache
    # 'mknginxpagespeed': ('mknginxpagespeed', 'alpine:3.8', 'core'),

    # nut
    'mknut': ('mknut', 'alpine:3.17', 'core'),

    # download open library dump of ids in database and insert into downloads - run and exit
    'mkopenlibrarynetfetchbulk': ('mkopenlibrarynetfetchbulk', 'scratch', 'core'),

    # since using pgpool in sqlx for all containers, I don't think this has a use atm
    # database connection pooler
    # 'mkpgbouncer': ('mkpgbouncer', 'alpine:3.17.1', 'core'),

    # consume and process ampq records
    'mkrabbitconsume': ('mkrabbitconsume', 'scratch', 'core'),

    # amqp service (rabbitmq)
    'mkrabbitmq': ('mkrabbitmq', 'alpine:3.11', 'core'),

    # schedulesdirect update
    'mkschedulesdirectupdate': ('mkschedulesdirectupdate', 'scratch', 'core'),

    # scan for network shares
    'mksharescanner': ('mksharescanner', 'scratch', 'core'),

    # download tmdb dump of ids in database and insert into downloads - run and exit
    'mktmdbnetfetchbulk': ('mktmdbnetfetchbulk', 'scratch', 'core'),

    # download tmdb dump of ids that were updated - run and exit
    'mktmdbnetfetchupdate': ('mktmdbnetfetchupdate', 'scratch', 'core'),

    # transcode/STREAM media to client - run and exit
    'mktranscode': ('mktranscode', 'scratch', 'core'),

    # transmission server
    'mktransmission': ('mktransmission', 'alpine:3.16.2', 'core'),

    # tvheadend
    'mktvheadend': ('mktvheadend', 'alpine:3.12', 'core'),

    # website via rust and axum
    'mkwebaxum': ('mkwebaxum', 'scratch', 'core'),

    # website via rust and rocket
    # 'mkwebrocket': ('mkwebrocket', 'busybox:1.36.0-uclibc', 'core'),
}

STAGE_ONE_GAME_SERVERS = {
    # for hosting games via dosbox and web
    'mkgamebasedosboxweb': ('mkgamebasedosboxweb', 'ubuntu:22.10', 'game_base'),

    # for hosting games via retroarch and web
    'mkgamebaseretroarchweb': ('mkgamebaseretroarchweb', 'debian:buster-slim', 'game_base'),

    # for hosting games via steamcmd
    'mkgamebasesteamcmd': ('mkgamebasesteamcmd', 'debian:10.9-slim', 'game_base'),

    # for hosting games via steamcmd
    'mkgamebasesteamcmdbullseye': ('mkgamebasesteamcmdbullseye', 'debian:bullseye-slim', 'game_base'),

    # for hosting games via steamcmd as root
    'mkgamebasesteamcmdroot': ('mkgamebasesteamcmdroot', 'debian:10.9-slim', 'game_base'),

    # for hosting games via steamcmd as root
    'mkgamebasesteamcmdbullseyeroot': ('mkgamebasesteamcmdbullseyeroot', 'debian:bullseye-slim', 'game_base'),

    # for hosting software via wine
    'mkgamebasewine': ('mkgamebasewine', 'debian:10.9-slim', 'game_base'),
}

STAGE_TWO_GAME_SERVERS = {
    # Battlefield 1942
    'mkgamebf42': ('mkgamebf42', 'ubuntu:14.04', 'game_server'),

    # Factorio
    'mkgamefactorio': ('mkgamefactorio', 'ubuntu:14.04', 'game_server'),

    # FAKE
    'mkgamekerbalspaceprogram': ('mkgamekerbalspaceprogram', 'FAKE', 'game_server'),

    # Minecraft
    'mkgameminecraft': ('mkgameminecraft', 'FAKE', 'game_server'),

    # Quake 3 Arena
    'mkgameq3a': ('mkgameq3a', 'FAKE', 'game_server'),

    # FAKE
    'mkgameq3a_cpma': ('mkgameq3a_cpma', 'FAKE', 'game_server'),

    # FAKE
    'mkgameq3a_osp': ('mkgameq3a_osp', 'FAKE', 'game_server'),

    # FAKE
    'mkgameq3a_rq3': ('mkgameq3a_rq3', 'FAKE', 'game_server'),

    # Quake 2
    'mkgamequake2': ('mkgamequake2', 'FAKE', 'game_server'),

    # Quake 4
    'mkgamequake4': ('mkgamequake4', 'FAKE', 'game_server'),

    # Quake Live
    'mkgamequakelive': ('mkgamequakelive', 'FAKE', 'game_server'),

    # Arma 3
    'mkgamesteamcmd_arma3': ('mkgamesteamcmd_arma3', 'FAKE', 'game_server'),

    # CS:Go
    'mkgamesteamcmd_csgo': ('mkgamesteamcmd_csgo', 'FAKE', 'game_server'),

    # FAKE
    'mkgamesteamcmd_doubleaction': ('mkgamesteamcmd_doubleaction', 'FAKE', 'game_server'),

    # Fist Full of Frags
    'mkgamesteamcmd_fistfuloffrags': ('mkgamesteamcmd_fistfuloffrags', 'FAKE', 'game_server'),

    # FAKE
    'mkgamesteamcmd_holdfastnaw': ('mkgamesteamcmd_holdfastnaw', 'FAKE', 'game_server'),

    # FAKE
    'mkgamesteamcmd_insurgency': ('mkgamesteamcmd_insurgency', 'FAKE', 'game_server'),

    # FAKE
    'mkgamesteamcmd_mordhau': ('mkgamesteamcmd_mordhau', 'FAKE', 'game_server'),

    # FAKE
    'mkgamesteamcmd_squad': ('mkgamesteamcmd_squad', 'FAKE', 'game_server'),

    # Team Fortress II
    'mkgamesteamcmd_tf2': ('mkgamesteamcmd_tf2', 'FAKE', 'game_server'),

    # Valheim
    'mkgamesteamcmd_valheim': ('mkgamesteamcmd_valheim', 'FAKE', 'game_server'),

    # UT99
    'mkgameut99': ('mkgameut99', 'FAKE', 'game_server'),

    # UT2004
    'mkgameut2004': ('mkgameut2004', 'FAKE', 'game_server'),

    # FAKE
    'mkgamewindward': ('mkgamewindward', 'FAKE', 'game_server'),
}

STAGE_ONE_OPTIONS = {
    'mkmoosefscgi': ('mkmoosefscgi', 'debian:buster', 'option'),
    'mkmoosefschunkserver': ('mkmoosefschunkserver', 'debian:buster', 'option'),
    'mkmoosefsmaster': ('mkmoosefsmaster', 'debian:buster', 'option'),
    'mkmoosefsmetalogger': ('mkmoosefsmetalogger', 'debian:buster', 'option'),
}

STAGE_TWO_OPTIONS = {

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
