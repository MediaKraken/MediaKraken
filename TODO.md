remove section from user home page "All caught up No new media. Enjoy your library."

verify mkshedulesdirect against their current api

design/code page for live tv using normal grid pattern that most apps use (channels, times, etc)

readd live tv icon on main user page after tv shows, use television_live.png

readd home media on main user page after music vidoes, use vi_camera.png as icon

readd cctv on main user page, last itm in kist, use surveillance-cctv-symbol-in-rounded-triangle.png

find the latest https://data.metabrainz.org/pub/musicbrainz/data/fullexport/20261125-002239/mbdump.tar.bz2
    unzip/untar the file to /mediakraken

********** don't do  below this line**********
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

