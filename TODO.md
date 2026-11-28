mkwebaxum
    admin pages no longer show proper login status (shows login/signup when I'm signed in)
    https://mk.mediakraken.media/admin/hardware needs a submit scan button and a text input that can accept the subnet to scan via a rabbitmq json to mkhardwarescanner with the subnet in "Subnet"
    metadata music still 500's
    this 500's nonsense needs to stop..catch freakn errors and send to loki with an actual debug message so actaul troubleshooting can happen without guessing nonstop and wasting time - all endpoints
    admin page user page populate is admin using mm_axum_user_permissions
        remove LANG column
        true/false for active if they are signed in
        code up the update user button
        code up the delete user button
    CloudNativePG Backups
        Unable to load CloudNativePG backups: HTTP status client error (403 Forbidden) for url (https://kubernetes.default.svc/apis/postgresql.cnpg.io/v1/namespaces/cnpg-system/backups)
    metadata movie detail page....if Backdrop is populated, show in the background
    metadata movie filter by genre is very slow.....there is an index on the json..any thoughts on speeding it up

mkhardwarescanner
    need to get subnet to scan via "Subnet" in the rabbitmq json message
    all hardware scans need to scan the above subnet
    uncomment all hardware scans and compile

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

