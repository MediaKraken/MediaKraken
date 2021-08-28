
pub async fn nfo_xml_file(media_file_path: &str) {
    let mut nfo_data;
    let mut mutxml_data;
    // check for NFO or XML as no need to do lookup if ID found in it
    // pull the "real" extension
    let ext_check = pathlib.Path(media_file_path).suffix.lower();
    if ext_check in common_file_extentions.SUBTITLE_EXTENSION {
        // need to chop off the lang too, the split works even with no .lang in name
        let nfo_file_check = media_file_path.rsplit(".", 2)[0] + ".nfo";
        let xml_file_name = media_file_path.rsplit(".", 2)[0] + ".xml";
        }
    else {
        // not a subtitle, should be a "normal" file
        let nfo_file_check = media_file_path.rsplit(".", 1)[0] + ".nfo";
        let xml_file_name = media_file_path.rsplit(".", 1)[0] + ".xml";
        }
    if os.path.isfile(nfo_file_check) {
        // check for nfo
            let nfo_data = xmltodict.parse(common_file.com_file_load_data(nfo_file_check, False));
        }
    else {
        // only check for xml if nfo doesn"t exist
        if os.path.isfile(xml_file_name) {
            // check for xml
                let xml_data = xmltodict.parse(common_file.com_file_load_data(xml_file_name, False));
                }
        else { if os.path.isfile(
                os.path.join(os.path.dirname(os.path.abspath(media_file_path)), "movie.xml")) {
                let xml_data = xmltodict.parse(common_file.com_file_load_data(os.path.join(
                    os.path.dirname(os.path.abspath(media_file_path)), "movie.xml"), False));
              }  }}
    return nfo_data, xml_data
 }