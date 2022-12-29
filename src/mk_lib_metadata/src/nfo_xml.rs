#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

/*

pub async fn nfo_file_tv(media_file_path):
    """
    Find and load nfo and xml file(s) if they exist
    """
    nfo_data = None
    // check for NFO or XML as no need to do lookup if ID found in it
    // TODO should check for one dir back too I suppose
    nfo_file_check = media_file_path.rsplit('/', 1)[0] + 'tvinfo.nfo'
    if os.path.isfile(nfo_file_check):  // check for nfo
        try:
            nfo_data = xmltodict.parse(common_file.com_file_load_data(nfo_file_check, False))
        except xml.parsers.expat.ExpatError:
            pass
        except UnicodeDecodeError:
            pass
    else:
        nfo_file_check = media_file_path.rsplit('/', 1)[0] + 'tvshow.nfo'
        if os.path.isfile(nfo_file_check):  // check for nfo
            try:
                nfo_data = xmltodict.parse(common_file.com_file_load_data(nfo_file_check, False))
            except xml.parsers.expat.ExpatError:
                pass
            except UnicodeDecodeError:
                pass
    return nfo_data


pub async fn nfo_xml_id_lookup(nfo_data, xml_data):
    """
    Lookup by id's in nfo/xml files
    """
    imdb_id = None
    tmdb_id = None
    // load both fields for more data in media_id_json on db
    if nfo_data != None:
        try:  // not all will have imdb
            imdb_id = nfo_data["movie"]["imdbid"]
            if len(imdb_id) == 0:
                imdb_id = None
        except KeyError:
            pass
        try:  // not all nfo's have the movie/tmdb
            tmdb_id = nfo_data["movie"]["tmdbid"]
            if len(tmdb_id) == 0:
                tmdb_id = None
        except KeyError:
            pass
    if xml_data != None:
        if "movie" in xml_data:  # standard nfo/xml file
            if imdb_id is None:
                try:  // not all xmls's will have the imdb
                    imdb_id = xml_data["movie"]["imdbid"]
                    if len(imdb_id) == 0:
                        imdb_id = None
                except KeyError:
                    pass
            if tmdb_id is None:
                try:  // not all xml's have the movie/tmdb
                    tmdb_id = xml_data["movie"]["tmdbid"]
                    if len(tmdb_id) == 0:
                        tmdb_id = None
                except KeyError:
                    pass
        else:  // movie.xml
            if imdb_id is None:
                try:  // not all xmls's will have the imdb
                    imdb_id = xml_data["Title"]["IMDB"]
                    if len(imdb_id) == 0:
                        imdb_id = None
                except KeyError:
                    pass
            if tmdb_id is None:
                try:  // not all xml's have the movie/tmdb
                    tmdb_id = xml_data["Title"]["TMDbId"]
                    if len(tmdb_id) == 0:
                        tmdb_id = None
                except:
                    pass
    return imdb_id, tmdb_id


pub async fn nfo_id_lookup_tv(nfo_data):
    """
    Look up id's in nfo/xml lookup for tv
    """
    imdb_id = None
    tmdb_id = None
    // load both fields for more data in media_id_json on db
    if nfo_data != None:
        try:
            tmdb_id = nfo_data["episodedetails"]["tmdbid"]
        except KeyError:
            pass
        try:
            imdb_id = nfo_data["episodedetails"]["imdbid"]
        except KeyError:
            pass
    return imdb_id, tmdb_id

 */
