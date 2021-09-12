
/*
# determine video attributes
def com_ffmpeg_media_attr(file_path):
    """
    Runs ffprobe to generate the media file specifications which is returned in json
    """
    try:
        media_json = subprocess.check_output(
            shlex.split('ffprobe -hide_banner -show_format -show_streams'
                        ' -show_chapters -print_format json \"%s\"', (file_path,)))
    except:
        return None
    return media_json.decode('utf-8')

 */