mkwebaxum
    in admin, database page.  split out the record count and size into the 3 schemas
        mediakraken, musicbrainz and covert_art_archive
        then have a grand total of all three together at the bottom

    remove hostip from server info in admin dashboard

    admin dashboard, in active users....show data of users who are currently signed in.

    admin dashboard, backups.....show available backups for cnpg

    admin dashboadm reporting: break u into two seperate pages
        known media - which is what displays now
        unmatched media - files that are in mm_media but no metadata attached (ie, not id'd)

    admin upc import
        allow admin to add upc via scanner or pasting codes into textbox
        allow admin to upload file with upc codes

    admin page, settings, need to be able to save the settings to mm_options_and_status, mm_options_json

    iradio page.....selecting new page shouldn't kill the playing station

    iradio page...when selecting play from list....should autostart the stream playback

    https://mk.mediakraken.media/user/search should default to all, and don't reset the type after each search

    global search doesn't seem to return any values

    https://mk.mediakraken.media/user/hardware
        like the Philips Hue section add the following sections
        add Chromecast
        add HDHomeRun
        add GlobalCache
        add DNLA
        add OnVif Cameras

mk_lib_metadata
    in upcmdb.rs add code to do all supported api calls for url in the file
    in upcdiscdb.rs add code to submit ups and screen srape the results and return json
    in ean-db.rs add code to do all the support api alls for url in the file

********** don't do  below this line**********
kubectl -n mediakraken create secret generic db-password-admin \
  --from-literal=username=postgres \
  --from-literal=password=metamanadmin

apiVersion: v1
kind: Secret
metadata:
  name: db-password-admin
  namespace: mediakraken
type: Opaque
stringData:
  username: postgres
  password: metamanadmin

change cnpg setup/config to create schema musicbrainz owned by metaman
    CREATE SCHEMA musicbrainz; <- do by hand
    build usage rights for metaman
    SET search_path = musicbrainz, mediakraken, public

CREATE SCHEMA mediakraken;

recommended settings for browser stream
for browser playback
    ffmpeg -i "/mnt/share/MediaKraken_Test_Media/Movie/16_BLOCKS (2006)/extras/16 Blocks (2006).mkv" \
        -map 0:v:0 -map 0:a:0 \
        -c:v libx264 -preset fast -crf 20 -pix_fmt yuv420p \
        -g 60 -keyint_min 60 -sc_threshold 0 \
        -c:a aac -b:a 192k -ac 2 \
        -f hls -hls_time 4 -hls_playlist_type vod \
        -hls_segment_type fmp4 -hls_fmp4_init_filename init.mp4 \
        /var/www/html/stream/16-blocks/index.m3u8


recommneded settings for browser stream with nvec
    ffmpeg -hwaccel cuda -hwaccel_output_format cuda \
        -i "/mnt/share/MediaKraken_Test_Media/Movie/16_BLOCKS (2006)/extras/16 Blocks (2006).mkv" \
        -map 0:v:0 -map 0:a:0 \
        -vf "hwdownload,format=nv12,setsar=853/720,format=yuv420p" \
        -c:v h264_nvenc -preset p4 -rc vbr -cq 20 \
        -g 60 -keyint_min 60 -sc_threshold 0 \
        -c:a aac -b:a 192k -ac 2 \
        -f hls -hls_time 4 -hls_playlist_type vod \
        -hls_segment_type fmp4 -hls_fmp4_init_filename init.mp4 \
        /var/www/html/stream/16-blocks/index.m3u8

