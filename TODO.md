complete rebuild of mkmusicbrainz
    1) I'll setup new env to allow postgres user to be able to create schema/etc
        POSTGRES_USER_ADMIN
        POSTGRES_PASSWORD_ADMIN
    2) Initial/Update are still driven by rabbitmq message
    3) create musicbrainz schema if not exists in mkdatabase
    4) user metaman will be the owner of the new schema
    5) using file located https://data.metabrainz.org/pub/musicbrainz/data/fullexport/LATEST
        download/unarchive the following files..then run all the files in them to populate postgresql
        mbdump.tar.bz2
        mbdump-derived.tar.bz2
        mbdump-cdstubs.tar.bz2
        mbdump-cover-art-archive.tar.bz2


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
  username: postgress
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

