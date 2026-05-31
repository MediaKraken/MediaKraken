// https://developers.themoviedb.org/3

use crate::image_path;
use mk_lib_common::mk_lib_common_enum_media_type;
use mk_lib_database;
use mk_lib_image;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use sqlx::types::Uuid;
use std::env;
use tokio::time::{sleep, Duration};
use torrent_name_parser::Metadata;

const TMDB_RATE_LIMIT_STATUS_CODE: i64 = 25;
const TMDB_RATE_LIMIT_RETRY_ATTEMPTS: u8 = 5;
const TMDB_RATE_LIMIT_RETRY_DELAY_SECONDS: u64 = 2;

fn debug_logging_enabled() -> bool {
    env::var("DEBUG").ok().as_deref() == Some("true")
}

async fn tmdb_fetch_json_with_retry(
    url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    for attempt in 0..TMDB_RATE_LIMIT_RETRY_ATTEMPTS {
        let response = mk_lib_network::mk_data_from_url_to_json(url.clone()).await?;
        let is_rate_limited = response
            .get("status_code")
            .and_then(serde_json::Value::as_i64)
            .map(|code| code == TMDB_RATE_LIMIT_STATUS_CODE)
            .unwrap_or(false);

        if !is_rate_limited {
            return Ok(response);
        }

        if attempt + 1 < TMDB_RATE_LIMIT_RETRY_ATTEMPTS {
            sleep(Duration::from_secs(TMDB_RATE_LIMIT_RETRY_DELAY_SECONDS)).await;
        }
    }

    Err("TMDB rate limit persisted after retries".into())
}

pub async fn provider_tmdb_movie_fetch(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    metadata_uuid: Uuid,
    tmdb_api_key: &str,
)-> Result<(), Box<dyn std::error::Error>> {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_movie_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        ?;
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skip Movie: {}", tmdb_id);
        return Ok(());
    }
    let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await?;
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
    if result_json.get("belongs to collection").is_some() {
        // TODO check for and insert collections fetch record
    }
    Ok(())
}

pub async fn provider_tmdb_person_fetch(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    metadata_uuid: Uuid,
    tmdb_api_key: &str,
)-> Result<(), Box<dyn std::error::Error>> {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_person_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        ?;
    if debug_logging_enabled() {
        mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
            json!({ "Type": "Person", "Module": std::module_path!(), "Result": result_json }),
        )
        .await
        ?;
    }
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skip Person: {}", tmdb_id);
        return Ok(());
    }
    if debug_logging_enabled() {
        mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
            json!({ "Type": "Person After", "Module": std::module_path!() }),
        )
        .await
        ?;
    }
    let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await?;
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert(
        sqlx_pool,
        metadata_uuid,
        tmdb_id,
        &result_json,
        image_json,
    )
    .await;
    Ok(())
}

pub async fn provider_tmdb_tv_fetch(
    sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    metadata_uuid: Uuid,
    tmdb_api_key: &str,
)-> Result<(), Box<dyn std::error::Error>> {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_tv_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        ?;
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skip TV: {}", tmdb_id);
        return Ok(());
    }
    let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await?;
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
    Ok(())
}

pub async fn provider_tmdb_collection_fetch(
    _sqlx_pool: &sqlx::PgPool,
    tmdb_id: i32,
    _metadata_uuid: Uuid,
    tmdb_api_key: &str,
)-> Result<(), Box<dyn std::error::Error>> {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_collection_fetch_by_id(tmdb_id, tmdb_api_key)
        .await
        ?;
    if result_json.get("success").is_some() && result_json["success"] == false {
        println!("Skip Collection: {}", tmdb_id);
        return Ok(());
    }
    // let image_json: serde_json::Value = provider_tmdb_meta_info_build(&result_json).await.unwrap();
    // let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_collection::mk_lib_database_meta_collection_insert(
    //     sqlx_pool,
    //     metadata_uuid,
    //     tmdb_id,
    //     &result_json,
    //     image_json,
    // )
    // .await;
    Ok(())
}

pub async fn provider_tmdb_movie_id_max(
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/movie/latest?api_key={}",
        api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_person_id_max(
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/person/latest?api_key={}",
        api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_tv_id_max(
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/tv/latest?api_key={}",
        api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_collection_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/collection/{}?api_key={}",
        tmdb_id, api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_movie_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
        tmdb_id, api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_person_changes(
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/person/changes?api_key={}",
        api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_person_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/person/{}?api_key={}\
        &append_to_response=combined_credits,external_ids,images",
        tmdb_id, api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_review_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/review/{}?api_key={}",
        tmdb_id, api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_tv_fetch_by_id(
    tmdb_id: i32,
    api_key: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = tmdb_fetch_json_with_retry(format!(
        "https://api.themoviedb.org/3/tv/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
        tmdb_id, api_key
    ))
    .await
    ?;
    Ok(url_result)
}

pub async fn provider_tmdb_meta_info_build(
    result_json: &serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // create file path for poster
    let mut image_file_path = image_path::meta_image_file_path("poster".to_string())
        .await
        ?;
    let mut poster_file_path = String::new();
    let poster_path_value = result_json
        .get("poster_path")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            result_json
                .pointer("/images/profiles/0/file_path")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        });
    if let Some(poster_rel) = poster_path_value {
        image_file_path.push_str(&poster_rel);
        let _result = mk_lib_network::mk_download_file_from_url(
            format!("https://image.tmdb.org/t/p/original{}", poster_rel),
            &image_file_path,
        )
        .await;
        let _result = mk_lib_image::mk_lib_image::mk_image_file_thumb(&image_file_path);
        poster_file_path = image_file_path.clone();
    }
    // create file path for backdrop
    image_file_path = image_path::meta_image_file_path("backdrop".to_string())
        .await
        ?;
    let mut backdrop_file_path = String::new();
    if let Some(backdrop_rel) = result_json
        .get("backdrop_path")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
    {
        image_file_path.push_str(&backdrop_rel);
        let _result = mk_lib_network::mk_download_file_from_url(
            format!("https://image.tmdb.org/t/p/original{}", backdrop_rel),
            &image_file_path,
        )
        .await;
        backdrop_file_path = image_file_path;
    }
    // set local image json
    if !poster_file_path.trim().is_empty() {
        poster_file_path = poster_file_path.replace("/mediakraken/metadata", "");
    }
    if !backdrop_file_path.trim().is_empty() {
        backdrop_file_path = backdrop_file_path.replace("/mediakraken/metadata", "");
    }
    let image_json = json!({
    "Backdrop": backdrop_file_path,
    "Poster": poster_file_path
    });
    Ok(image_json)
}

pub async fn provider_tmdb_search(guessit_data: Metadata, media_type: i16, tmdb_api_key: &str) {
    let mut raw_query = guessit_data.title().to_string();
    if let Some(year) = guessit_data.year() {
        raw_query.push(' ');
        raw_query.push_str(&year.to_string());
    }
    let search_text: String =
        url::form_urlencoded::byte_serialize(raw_query.as_bytes()).collect();
    match media_type {
        mk_lib_common_enum_media_type::DLMediaType::MOVIE
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME
        | mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER => {
            let _url_result = tmdb_fetch_json_with_retry(format!(
                "https://api.themoviedb.org/3/search/movie\
                ?api_key={}&include_adult=1&query={}",
                tmdb_api_key, search_text
            ))
            .await;
        }
        mk_lib_common_enum_media_type::DLMediaType::TV
        | mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
        | mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS
        | mk_lib_common_enum_media_type::DLMediaType::TV_SEASON
        | mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE
        | mk_lib_common_enum_media_type::DLMediaType::TV_THEME
        | mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER => {
            let _url_result = tmdb_fetch_json_with_retry(format!(
                "https://api.themoviedb.org/3/search/tv\
                ?api_key={}&include_adult=1&query={}",
                tmdb_api_key, search_text
            ))
            .await;
        }
        mk_lib_common_enum_media_type::DLMediaType::PERSON => {
            let _url_result = tmdb_fetch_json_with_retry(format!(
                "https://api.themoviedb.org/3/search/person\
                ?api_key={}&include_adult=1&query={}",
                tmdb_api_key, search_text
            ))
            .await;
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
