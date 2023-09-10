// https://developers.themoviedb.org/3

use crate::image_path;
use mk_lib_common::mk_lib_common_enum_media_type;
use mk_lib_database;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use sqlx::types::Uuid;
use torrent_name_parser::Metadata;

pub async fn provider_tmdb_movie_fetch(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    metadata_uuid: Uuid,
    tmdb_api_key: &str,
) {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_movie_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        .unwrap();
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skipn Movie: {}", tmdb_id);
        return;
    }
    let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await.unwrap();
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_insert(
        sqlx_pool,
        metadata_uuid,
        tmdb_id,
        &result_json,
        image_json,
    )
    .await;
    if result_json.get("credits").is_some() {
        // cast/crew doesn't exist on all media
        if result_json["credits"].get("cast").is_some() {
            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(
                sqlx_pool,
                &result_json["credits"]["cast"],
            )
            .await;
        }

        if result_json["credits"].get("crew").is_some() {
            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(
                sqlx_pool,
                &result_json["credits"]["crew"],
            )
            .await;
        }
    }
}

pub async fn provider_tmdb_person_fetch(
    _sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    _metadata_uuid: Uuid,
    tmdb_api_key: &str,
) {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_person_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        .unwrap();
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skipn Person: {}", tmdb_id);
        return;
    }        
}

pub async fn provider_tmdb_tv_fetch(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    metadata_uuid: Uuid,
    tmdb_api_key: &str,
) {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_tv_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        .unwrap();
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skipn TV: {}", tmdb_id);
        return;
    }    
    let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await.unwrap();
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_tv_insert(
        sqlx_pool,
        metadata_uuid,
        tmdb_id,
        &result_json,
        image_json,
    )
    .await;
    if result_json.get("credits").is_some() {
        // cast/crew doesn't exist on all media
        if result_json["credits"].get("cast").is_some() {
            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(
                sqlx_pool,
                &result_json["credits"]["cast"],
            )
            .await;
        }

        if result_json["credits"].get("crew").is_some() {
            mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(
                sqlx_pool,
                &result_json["credits"]["crew"],
            )
            .await;
        }
    }
}

pub async fn provider_tmdb_movie_id_max(
    api_key: &String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/movie/latest?api_key={}",
        api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_id_max(
    api_key: &String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/person/latest?api_key={}",
        api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_tv_id_max(
    api_key: &String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/tv/latest?api_key={}",
        api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_collection_fetch_by_id(
    tmdb_id: i32,
    api_key: &String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/collection/{}?api_key={}",
        tmdb_id, api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_movie_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
        tmdb_id, api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_changes(
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/person/changes?api_key={}",
        api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/person/{}?api_key={}\
        &append_to_response=combined_credits,external_ids,images",
        tmdb_id, api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_review_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/review/{}?api_key={}",
        tmdb_id, api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_tv_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.themoviedb.org/3/tv/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
        tmdb_id, api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_meta_info_build(
    result_json: &serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // create file path for poster
    let mut image_file_path = image_path::meta_image_file_path("poster".to_string())
        .await
        .unwrap();
    let mut poster_file_path = String::new();
    if result_json.get("poster_path").is_some() && !result_json["poster_path"].is_null() {
        image_file_path += &result_json["poster_path"].as_str().unwrap().to_string();
        println!("ifilepath {}", image_file_path);
        let _result = mk_lib_network::mk_download_file_from_url(
            format!(
                "https://image.tmdb.org/t/p/original{}",
                &result_json["poster_path"].as_str().unwrap().to_string()
            ),
            &image_file_path,
        )
        .await;
        poster_file_path = image_file_path;
    }
    // create file path for backdrop
    image_file_path = image_path::meta_image_file_path("backdrop".to_string())
        .await
        .unwrap();
    let mut backdrop_file_path = String::new();
    if result_json.get("backdrop_path").is_some() && !result_json["backdrop_path"].is_null() {
        image_file_path += &result_json["backdrop_path"].as_str().unwrap().to_string();
        println!("iifilepath {}", image_file_path);
        let _result = mk_lib_network::mk_download_file_from_url(
            format!(
                "https://image.tmdb.org/t/p/original{}",
                &result_json["backdrop_path"].as_str().unwrap().to_string()
            ),
            &image_file_path,
        )
        .await;
        backdrop_file_path = image_file_path;
    }
    // set local image json
    if !poster_file_path.trim().is_empty() {
        poster_file_path = poster_file_path.replace("/mediakraken/static", "");
    }
    if !backdrop_file_path.trim().is_empty() {
        backdrop_file_path = backdrop_file_path.replace("/mediakraken/static", "");
    }
    let image_json = json!({
    "Backdrop": backdrop_file_path,
    "Poster": poster_file_path
    });
    Ok(image_json)
}

pub async fn provider_tmdb_search(guessit_data: Metadata, media_type: i16, tmdb_api_key: &String) {
    let mut search_text: String = guessit_data.title().to_string().replace(" ", "%20");
    if guessit_data.year().is_some() {
        search_text = format!(
            "{}%20{}",
            search_text,
            guessit_data.year().unwrap().to_string()
        );
    }
    match media_type {
        mk_lib_common_enum_media_type::DLMediaType::MOVIE
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER => {
            let _url_result = mk_lib_network::mk_data_from_url_to_json(format!(
                "https://api.themoviedb.org/3/search/movie\
                ?api_key={}&include_adult=1&query={}",
                search_text, tmdb_api_key
            ))
            .await
            .unwrap();
        }
        mk_lib_common_enum_media_type::DLMediaType::TV
        | mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
        | mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS
        | mk_lib_common_enum_media_type::DLMediaType::TV_SEASON
        | mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE
        | mk_lib_common_enum_media_type::DLMediaType::TV_THEME
        | mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER => {
            let _url_result = mk_lib_network::mk_data_from_url_to_json(format!(
                "https://api.themoviedb.org/3/search/tv\
                ?api_key={}&include_adult=1&query={}",
                search_text, tmdb_api_key
            ))
            .await
            .unwrap();
        }
        mk_lib_common_enum_media_type::DLMediaType::PERSON => {
            let _url_result = mk_lib_network::mk_data_from_url_to_json(format!(
                "https://api.themoviedb.org/3/search/person\
                ?api_key={}&include_adult=1&query={}",
                search_text, tmdb_api_key
            ))
            .await
            .unwrap();
        }
        _ => eprintln!("provider_tmdb_search type does not equal any value"),
    }
}

/*
        search_json = search_json.json()
        if search_json != None and search_json['total_results'] > 0:
            for res in search_json['results']:
                await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                    message_type='info',
                    message_text={
                        "result": res['title'],
                        'id': res['id'],
                        'date':
                            res[
                                'release_date'].split(
                                '-',
                                1)[
                                0]})
                if media_year != None and type(media_year) is not list \
                        and (str(media_year) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) - 1) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) - 2) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) - 3) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) + 1) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) + 2) == res['release_date'].split('-', 1)[0]
                             or str(int(media_year) + 3) == res['release_date'].split('-', 1)[0]):
                    if not id_only:
                        return 'info', self.com_tmdb_metadata_by_id(res['id'])
                    else:
                        return 'idonly', res['id']
            return None, None
        else:
            return None, None

    pub async fn com_tmdb_meta_bio_image_build(self, result_json):
        """
        # download info and set data to be ready for insert into database
        """
        // create file path for poster
        image_file_path = await common_metadata.com_meta_image_file_path(result_json['name'],
                                                                         'person')
        if 'profile_path' in result_json and result_json['profile_path'] != None:
            if not os.path.isfile(image_file_path + result_json['profile_path']):
                if result_json['profile_path'] != None:
                    if not os.path.isfile(image_file_path):
                        await common_network_async.mk_network_fetch_from_url_async(
                            'https://image.tmdb.org/t/p/original' + result_json['profile_path'],
                            image_file_path + result_json['profile_path'])
        // set local image json
        return image_file_path.replace(common_global.static_data_directory, '')

pub async fn movie_search_tmdb(db_connection, file_name):
    """
    # search tmdb
    """
    // TODO aren't I doing two guessits per file name then?
    file_name = guessit(file_name)
    if type(file_name['title']) == list:
        file_name['title'] = common_string.com_string_guessit_list(file_name['title'])
    metadata_uuid = None
    // try to match ID ONLY
    if 'year' in file_name:
        match_response, match_result = await common_global.api_instance.com_tmdb_search(
            file_name["title"], file_name["year"], id_only=true,
            media_type=common_global.DLMediaType.Movie.value)
    else:
        match_response, match_result = await common_global.api_instance.com_tmdb_search(
            file_name['title'], None, id_only=true,
            media_type=common_global.DLMediaType.Movie.value)
    if match_response == 'idonly':
        // check to see if metadata exists for TMDB id
        metadata_uuid = await db_connection.db_meta_guid_by_tmdb(match_result)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "meta movie db result": metadata_uuid})
    else if match_response == 'info':
        // store new metadata record and set uuid
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "meta movie movielookup info "
                                                                             "results": match_result})
    else if match_response == 're':
        // multiple results
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "movielookup multiple results":
                                                                                 match_result})
    return metadata_uuid, match_result

pub async fn movie_fetch_save_tmdb_review(db_connection, tmdb_id):
    """
    # grab reviews
    """
    review_json = await common_global.api_instance.com_tmdb_meta_review_by_id(tmdb_id)
    // review record doesn't exist on all media
    if review_json != None and review_json["total_results"] > 0:
        review_json_id = ({'themoviedb': str(review_json['id'])})
        await db_connection.db_review_insert(review_json_id,
                                             {'themoviedb': review_json})


pub async fn movie_fetch_save_tmdb_collection(db_connection, tmdb_collection_id, download_data):
    """
    # grab collection
    """
    // store/update the record
    // don't string this since it's a pure result store
    collection_guid = await db_connection.db_collection_by_tmdb(tmdb_collection_id)
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         "collection": tmdb_collection_id,
                                                                         'guid': collection_guid})
    if collection_guid is None:
        // insert
        collection_meta = common_global.api_instance.com_tmdb_meta_collection_by_id(
            tmdb_collection_id)
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "col": collection_meta})
        // poster path
        if download_data["Poster"] != None:
            image_poster_path = common_metadata.com_meta_image_path(download_data['Name'],
                                                                    'poster', 'themoviedb',
                                                                    download_data['Poster'])
        else:
            image_poster_path = None
        // backdrop path
        if download_data["Backdrop"] != None:
            image_backdrop_path = common_metadata.com_meta_image_path(download_data['Name'],
                                                                      'backdrop', 'themoviedb',
                                                                      download_data['Backdrop'])
        else:
            image_backdrop_path = None
        await db_connection.db_collection_insert(download_data["Name"], download_data["GUID"],
                                                 collection_meta, {'Poster': image_poster_path,
                                                                   'Backdrop': image_backdrop_path})
        // commit all changes to db
        await db_connection.db_commit()
        return 1  # to add totals later
    else:
        // update
        // db_connection.db_collection_update(collection_guid, guid_list)
        return 0  # to add totals later

 */
