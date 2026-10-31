MediaKraken
    Audio/Video playback suite for web, client and mobile app playback.

Deployment is done via k8s.

Programs that run in K8s
    mkchatmumble
        Hosts mumble server
    mkchatteamspeak
        Hosts teamspeak server
    mkcron
        Runs defined jobs for housekeeping, data updates, etc
    mkdownload
        Download files and datasets
    mkfanotify
        File add, update and delete of file(s) notifications
    mkgamesdbnetfetchbulk
        (WIP) Consumes rabbitmq messages; intended to fetch game metadata (platforms, etc) from the gamesdb.net API
    mkglobalcache
        Grab manufacturer, models, etc of electronic equipment (redo to hit Wikipedia?)
    mkguessitrest
        Web API for determining proper Movie and TV Show name of a media file
    mkhardwarecontrol
        Receive and send hardware controls via network, IR, etc
    mkhardwarescanner
        Scan for and update hardware found on specified subnet
    mkinit
        Initializes database and folder file system for saving image metadata
    mkinotify
        File add, update and delete of file(s) notifications
    mkiradio
        Scrape/api for finding internet radio stations and update their url/description if needed
    mklibretrocorefetchupdate
        Grab latest libretro emulation cores
    mkmediascanner
        Scan directories for modified/added files and add/update database for found file(s)
    mkmetadata
        Identify media and download the appropriate metadata
    mkmetadatamame
        Fetch and update MAME/MESS game metadata
    mkmulticast
        Do multicast broadcasting for cluster discovery
    mkmusicbrainz
        Grab musicbrainz data and update/insert into DB
    mknut
        Hosts UPS software NUT
    mkopenlibrarynetfetchbulk
        Grab openlibrary metadata and update DB
    mkrabbitconsume
        Processes rabbitmq messages
    mkschedulesdirectupdate
        Fetch current TV information and update DB
    mksharescanner
        Scan for network SMB and NFS shares on specified subnet
    mktmdbnetfetchbulk
        Insert movie and TV tmdb ids to database for metadata fetching
    mktranscode
        Will do transcoding/streaming to web/devices that can't do direct play from the media file itself
    mktransmission
        WebUI and BitTorrent server
    mktvheadend
        Interface with TV/Streaming hardware
    mkwebaxum
        Main website of application suite

Libraries used to build
    mk_lib_common
        Shared helpers: media type enums, media extension lookups, date/bytesize/ffmpeg helpers, pagination, i18n, version and docker utilities
    mk_lib_compression
        Archive extraction: tar.gz, gzip, zip (in-process plus external gunzip/tar)
    mk_lib_database
        Postgres/SQLx data access layer: media catalog (movie, tv, music, game, book, ...), users, hardware, libraries, shares, search, backups, cron, schema versioning
    mk_lib_file
        File system and mtime helpers, plus Garage S3 object storage client (feature s3) and garage-admin (feature garage-admin)
    mk_lib_filler
        Empty lib to make chef happy
    mk_lib_hardware
        Driver/control integrations for AV and home hardware: chromecast, roku, firetv, appletv, lenbrook, marantz, onkyo, pioneer, hue, nut ups, hdhomerun, crestron, alexa, arduino, pi sensors
    mk_lib_hash
        Hashing helpers (blake3, crc32, ed2k, md5, sha1) including whole-file hashing
    mk_lib_image
        Image processing utilities and image-to-html helpers
    mk_lib_logging
        Log shipping to ELK (elasticsearch) and Loki
    mk_lib_metadata
        Media identification (guessit) and metadata build/parse (nfo xml, id3, m3u8) for movie, tv, music, game, book, anime, adult, sports, plus external provider clients (tmdb, thetvdb, musicbrainz, coverartarchive, comicvine, amazon, anidb, barcode lookups, ...)
    mk_lib_network
        Network utilities: http clients, email (aws), ftp, ssh, telnet, serial, ping, wol, upnp/dlna/ssdp, rss, transmission, openweather, schedules direct, kubernetes, ldap, rate limiting, mediakraken server discovery
    mk_lib_ocr
        OCR via tesseract and surya (text and confidence from images)
    mk_lib_rabbitmq
        RabbitMQ connect/consume/ack helpers
    mk_lib_share
        Network share access: SMB (via pavao and smbclient bindings) and file share helpers
    mk_lib_steam
        Steam Web API client: typed ISteam*/IPlayerService endpoints (rsteam re-export) plus generic api.steampowered.com calls
    mk_lib_xbox
        Xbox Live API client: auth (user token/xsts), gamertag->xuid resolution, people search, plus generic XBL endpoint calls with automatic XBL3.0 authorization header

External applications
    brother_print_label
        Print label for watched/dvd/bray/rating
    mediakraken
        Dioxus 0.7 desktop/mobile GUI client; talks to the mkwebaxum web backend over http (reqwest) with home, media browse/playback, hardware and scan (upc lookup) pages
    pi_audio
        Raspberry pi FLTK app to log scanned upc codes of owned media to a local sqlite db (work-in-progress audio capture variant of pi_upc)
    pi_upc
        Raspberry pi FLTK app to scan/enter upc codes and check known/owned counts against a local sqlite db
    pi_voice
        Raspberry pi voice capture client: records mic audio to wav (cpal), sends it to a vosk speech recognition web socket, and shows mediakraken results in an embedded webview
    theater_controller_pi
        Raspberry pi FLTK remote: category selection and play/pause/resume/stop sent over tcp to a theater_thin instance
    theater_full
        Raspberry pi FLTK theater client (800x480): discovers the mediakraken server and presents a category grid (movie, tv, games, music, live tv, home video, internet, ...)
    theater_roku
        Roku SceneGraph channel (BrightScript) to browse the mediakraken catalog by category from a roku device
    theater_thin
        Thin TV playback client: FLTK window embedding mpv (ipc over unix socket) that plays whatever the controller tells it to
    theater_tizen
        Tizen TV web app (html/js): mediakraken audio/video streaming client for tizen televisions
