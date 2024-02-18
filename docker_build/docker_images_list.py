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
DOCKER_IMAGES = {
    'RustBaseAlpine': ('mkbase_rust_alpine', 'base'),
    'RustBaseDebian': ('mkbase_rust_debian', 'base'),


    # barman postgresql backup server
    'mkbarman': ('mkbarman', 'core'),

    # chat server via mumble
    'mkchatmumble': ('mkchatmumble', 'core'),

    # chat server via ts3 - free license version
    'mkchatteamspeak': ('mkchatteamspeak', 'core'),

    # process cron jobs from the database to amqp or direct container launch
    'mkcron': ('mkcron', 'core'),

    # database via postgresql/citus
    'mkdatabase_citus': ('mkdatabase', 'core'),

    # download files/etc trailers/etc from ampq records
    'mkdownload': ('mkdownload', 'core'),

    # filebeat
    'mkfilebeat': ('mkfilebeat', 'core'),

    # thegamesdb bulk data fetch
    'mkgamesdbnetfetchbulk': ('mkgamesdbnetfetchbulk', 'core'),

    # download manufactuer stuff from global cache
    'mkglobalcache': ('mkglobalcache', 'core'),

    # guessit via web rest
    'mkguessitrest': ('mkguessitrest', 'core'),

    # runs control network/ir/if/etc
    'mkhardwarecontrol': ('mkhardwarecontrol', 'core'),

    # runs as HOST to find new hardware - run and exit
    'mkhardwarescanner': ('mkhardwarescanner', 'core'),

    # inotify of file system changes to amqp
    'mkinotify': ('mkinotify', 'core'),

    # download libretro cores that are newer - run and exit
    'mklibretrocorefetchupdate': ('mklibretrocorefetchupdate', 'core'),

    # scan media directories for new media - run and exit
    'mkmediascanner': ('mkmediascanner', 'core'),

    # process metadata for media
    'mkmetadata': ('mkmetadata', 'core'),

    # process metadata for mame and other game xml
    'mkmetadatamame': ('mkmetadatamame', 'core'),

    # "broadcast" multicast for discovery
    'mkmulticast': ('mkmulticast', 'core'),

    # musicbrainz load
    'mkmusicbrainz': ('mkmusicbrainz', 'core'),

    # nut
    'mknut': ('mknut', 'core'),

    # download open library dump of ids in database and insert into downloads - run and exit
    'mkopenlibrarynetfetchbulk': ('mkopenlibrarynetfetchbulk', 'core'),

    # consume and process ampq records
    'mkrabbitconsume': ('mkrabbitconsume', 'core'),

    # amqp service (rabbitmq)
    'mkrabbitmq': ('mkrabbitmq', 'core'),

    # schedulesdirect update
    'mkschedulesdirectupdate': ('mkschedulesdirectupdate', 'core'),

    # scan for network shares
    'mksharescanner': ('mksharescanner', 'core'),

    # download tmdb dump of ids in database and insert into downloads - run and exit
    'mktmdbnetfetchbulk': ('mktmdbnetfetchbulk', 'core'),

    # download tmdb dump of ids that were updated - run and exit
    'mktmdbnetfetchupdate': ('mktmdbnetfetchupdate', 'core'),

    # transcode/STREAM media to client - run and exit
    'mktranscode': ('mktranscode', 'core'),

    # transmission server
    'mktransmission': ('mktransmission', 'core'),

    # tvheadend
    'mktvheadend': ('mktvheadend', 'core'),

    # website via rust and axum
    'mkwebaxum': ('mkwebaxum', 'core'),


    # for hosting games via dosbox and web
    'mkgamebasedosboxweb': ('mkgamebasedosboxweb', 'game_base'),

    # for hosting games via retroarch and web
    'mkgamebaseretroarchweb': ('mkgamebaseretroarchweb', 'game_base'),

    # for hosting games via steamcmd
    'mkgamebasesteamcmd': ('mkgamebasesteamcmd', 'game_base'),

    # for hosting games via steamcmd
    'mkgamebasesteamcmdbullseye': ('mkgamebasesteamcmdbullseye', 'game_base'),

    # for hosting games via steamcmd as root
    'mkgamebasesteamcmdroot': ('mkgamebasesteamcmdroot', 'game_base'),

    # for hosting games via steamcmd as root
    'mkgamebasesteamcmdbullseyeroot': ('mkgamebasesteamcmdbullseyeroot', 'game_base'),

    # for hosting software via wine
    'mkgamebasewine': ('mkgamebasewine', 'game_base'),


    # Battlefield 1942
    'mkgamebf42': ('mkgamebf42', 'game_server'),

    # Factorio
    'mkgamefactorio': ('mkgamefactorio', 'game_server'),

    # FAKE
    'mkgamekerbalspaceprogram': ('mkgamekerbalspaceprogram', 'game_server'),

    # Minecraft
    'mkgameminecraft': ('mkgameminecraft', 'game_server'),

    # Quake 3 Arena
    'mkgameq3a': ('mkgameq3a', 'game_server'),

    # FAKE
    'mkgameq3a_cpma': ('mkgameq3a_cpma', 'game_server'),

    # FAKE
    'mkgameq3a_osp': ('mkgameq3a_osp', 'game_server'),

    # FAKE
    'mkgameq3a_rq3': ('mkgameq3a_rq3', 'game_server'),

    # Quake 2
    'mkgamequake2': ('mkgamequake2', 'game_server'),

    # Quake 4
    'mkgamequake4': ('mkgamequake4', 'game_server'),

    # Quake Live
    'mkgamequakelive': ('mkgamequakelive', 'game_server'),

    # Arma 3
    'mkgamesteamcmd_arma3': ('mkgamesteamcmd_arma3', 'game_server'),

    # CS:Go
    'mkgamesteamcmd_csgo': ('mkgamesteamcmd_csgo', 'game_server'),

    # FAKE
    'mkgamesteamcmd_doubleaction': ('mkgamesteamcmd_doubleaction', 'game_server'),

    # Fist Full of Frags
    'mkgamesteamcmd_fistfuloffrags': ('mkgamesteamcmd_fistfuloffrags', 'game_server'),

    # FAKE
    'mkgamesteamcmd_holdfastnaw': ('mkgamesteamcmd_holdfastnaw', 'game_server'),

    # FAKE
    'mkgamesteamcmd_insurgency': ('mkgamesteamcmd_insurgency', 'game_server'),

    # FAKE
    'mkgamesteamcmd_mordhau': ('mkgamesteamcmd_mordhau', 'game_server'),

    # FAKE
    'mkgamesteamcmd_squad': ('mkgamesteamcmd_squad', 'game_server'),

    # Team Fortress II
    'mkgamesteamcmd_tf2': ('mkgamesteamcmd_tf2', 'game_server'),

    # Valheim
    'mkgamesteamcmd_valheim': ('mkgamesteamcmd_valheim', 'game_server'),

    # UT99
    'mkgameut99': ('mkgameut99', 'game_server'),

    # UT2004
    'mkgameut2004': ('mkgameut2004', 'game_server'),

    # FAKE
    'mkgamewindward': ('mkgamewindward', 'game_server'),


    'mkmoosefscgi': ('mkmoosefscgi', 'option'),
    'mkmoosefschunkserver': ('mkmoosefschunkserver', 'option'),
    'mkmoosefsmaster': ('mkmoosefsmaster', 'option'),
    'mkmoosefsmetalogger': ('mkmoosefsmetalogger', 'option'),


    'mkelk': ('mkelk', 'test'),
    'mkftpserver': ('mkftpserver', 'test'),
    'mkjenkins': ('mkjenkins', 'test'),
    'mkselenium': ('mkselenium', 'test'),
    'mksonatype': ('mksonatype', 'test'),
}
