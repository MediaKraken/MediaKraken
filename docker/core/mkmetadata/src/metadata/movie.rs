#![cfg_attr(debug_assertions, allow(dead_code))]

use serde_json::json;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use std::error::Error;
use stdext::function_name;
use torrent_name_parser::Metadata;

use crate::mk_lib_logging;

use crate::mk_lib_database_metadata_download_queue;
use crate::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

#[path = "provider/imdb.rs"]
mod provider_imdb;

#[path = "provider/omdb.rs"]
mod provider_omdb;

#[path = "provider/tmdb.rs"]
mod provider_tmdb;

pub struct MetadataMovieLastLookup {
    metadata_last_id: Uuid,
    metadata_last_imdb: String,
    metadata_last_tmdb: String,
}

pub async fn metadata_movie_lookup(
    sqlx_pool: &sqlx::PgPool,
    download_data: &DBDownloadQueueByProviderList,
    file_name: Metadata,
) -> Result<Uuid, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // don't bother checking title/year as the main_server_metadata_api_worker does it already
    let mut metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later
    Ok(metadata_uuid)
}

/*
pub async fn metadata_movie_lookup(sqlx_pool: &sqlx::PgPool, dl_row, guessit_data) {

    // determine provider id's from nfo/xml if they exist
    (nfo_data, xml_data) = metadata_nfo_xml.nfo_xml_file(dl_row.get("mdq_path"));
    (imdb_id, tmdb_id) = metadata_nfo_xml.nfo_xml_id_lookup(nfo_data, xml_data);
    // if same as last, return last id and save lookup
    if (imdb_id != None && imdb_id == metadata_movie_lookup.metadata_last_imdb)
        || (tmdb_id != None && tmdb_id == metadata_movie_lookup.metadata_last_tmdb) {
        // don"t need to set last......since they are equal
        return metadata_movie_lookup.metadata_last_id;
    }
    // doesn't match last id"s so continue lookup
    // if ids from nfo/xml, query local db to see if exist
    if tmdb_id != None {
        let metadata_uuid = db_connection.db_meta_guid_by_tmdb(tmdb_id);
    }
    // keep these separate just in case imdb is there but tmdb isn"t
    if imdb_id != None && metadata_uuid == None {
        let metadata_uuid = db_connection.db_meta_guid_by_imdb(imdb_id);
    }
    // if ids from nfo/xml on local db
    if metadata_uuid == None {
        // check to see if id is known from nfo/xml but not in db yet so fetch data
        if tmdb_id != None || imdb_id != None {
            if tmdb_id != None {
                provider_id = str(tmdb_id);
            } else {
                provider_id = imdb_id;
            }
            dl_meta = db_connection.db_download_que_exists(dl_row.get("mdq_id"),
                                                           common_global.DLMediaType.Movie.value,
                                                           "themoviedb",
                                                           provider_id);
            if dl_meta == None {
                let metadata_uuid = dl_row.get("mdq_new_uuid");
                db_connection.db_begin();
                db_connection.db_download_update(guid = dl_row.get("mdq_id"),
                                                 status = "Fetch",
                                                 provider_guid = provider_id);
                // set provider last so it's not picked up by the wrong thread too early
                db_connection.db_download_update_provider("themoviedb",
                                                          dl_row.get("mdq_id"));
                db_connection.db_commit();
            } else {
                let metadata_uuid = dl_meta;
            }
        }
    }
    // leave this AFTER the dl check as it looks at tmdbid and imdb for values!
    if metadata_uuid == None {
        // no ids found on the local database so begin name/year searches
        // db lookup by name and year (if available)
        if "year" in file_name {
            let metadata_uuid = db_connection.db_find_metadata_guid(file_name["title"],
            file_name["year"]); }
        else {
            let metadata_uuid = db_connection.db_find_metadata_guid(file_name["title"], None);
        }
    }
    if metadata_uuid == None {
        let metadata_uuid = dl_row.get("mdq_new_uuid");
        // no matches by name/year on local database
        // search themoviedb since not matched above via DB or nfo/xml
        // save the updated status
        db_connection.db_begin();
        db_connection.db_download_update(guid = dl_row.get("mdq_id"),
                                         status = "Search");
        // set provider last so it"s not picked up by the wrong thread
        db_connection.db_download_update_provider("themoviedb", dl_row.get("mdq_id"));
        db_connection.db_commit();
    }
    // set last values to negate lookups for same title/show
    metadata_movie_lookup.metadata_last_id = metadata_uuid;
    metadata_movie_lookup.metadata_last_imdb = imdb_id;
    metadata_movie_lookup.metadata_last_tmdb = tmdb_id;
    Ok(metadata_uuid)
}
*/
