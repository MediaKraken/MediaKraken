#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://developers.themoviedb.org/3

use std::path::Path;
use sqlx::{types::Uuid, types::Json};
use serde_json::json;

#[path = "../image_path.rs"]
mod image_path;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

#[path = "../../mk_lib_database_metadata_movie.rs"]
mod mk_lib_database_metadata_movie;
#[path = "../../mk_lib_database_metadata_person.rs"]
mod mk_lib_database_metadata_person;
#[path = "../../mk_lib_database_metadata_tv.rs"]
mod mk_lib_database_metadata_tv;

pub struct TMDBAPI {
    pub tmdb_api_key: String,
}

pub async fn provider_tmdb_movie_fetch(pool: &sqlx::PgPool, tmdb_id: i32, metadata_uuid: Uuid) {
    // fetch and save json data via tmdb id
    let result_json = provider_tmdb_movie_fetch_by_id(tmdb_id).await;
    let mut series_id: serde_json::Value;
    let mut image_json: serde_json::Value;
    (series_id, result_json, image_json) = provider_tmdb_meta_info_build(result_json.json());
    mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_insert(pool,
                                                                          metadata_uuid,
                                                                          series_id,
                                                                          result_json["title"],
                                                                          result_json,
                                                                          image_json);
    if result_json.contains_key("credits") {  // cast/crew doesn't exist on all media
        if result_json["credits"].contains_key("cast") {
            mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(pool,
                                                                                              "themoviedb",
                                                                                              result_json["credits"]["cast"]);
        }
        if result_json["credits"].contains_key("crew") {
            mk_lib_database_metadata_person::mk_lib_database_metadata_person_insert_cast_crew(pool,
                                                                                              "themoviedb",
                                                                                              result_json["credits"]["crew"]);
        }
    }
}


/*
        // 504	Your request to the backend server timed out. Try again.
        if result_json.status_code == 504 {
            await asyncio.sleep(60)
            // redo fetch due to 504
            await movie_fetch_save_tmdb(db_connection, tmdb_id, metadata_uuid)
            }
        else if result_json.status_code == 200 {

        else if result_json.status_code == 404 {
            // TODO handle 404's better
            metadata_uuid = None
            }
    else {  // is this is None....
        metadata_uuid = None
        }
    return metadata_uuid
 */

pub async fn provider_tmdb_movie_id_max()
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/movie/latest?api_key={}",
                TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_id_max()
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/person/latest?api_key={}",
                TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_tv_id_max()
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/tv/latest?api_key={}",
                TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_collection_fetch_by_id(tmdb_id: i32)
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/collection/{}?api_key={}",
                tmdb_id, TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_movie_fetch_by_id(tmdb_id: i32)
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/movie/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
                tmdb_id, TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_changes()
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/person/changes?api_key={}",
                TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_person_fetch_by_id(tmdb_id: i32)
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/person/{}?api_key={}\
        &append_to_response=combined_credits,external_ids,images",
                tmdb_id, TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_review_fetch_by_id(tmdb_id: i32)
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/review/{}?api_key={}",
                tmdb_id, TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_tv_fetch_by_id(tmdb_id: i32)
    -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result =  mk_lib_network::mk_data_from_url_to_json(
        format!("https://api.themoviedb.org/3/tv/{}?api_key={}\
        &append_to_response=credits,reviews,release_dates,videos",
                tmdb_id, TMDBAPI.tmdb_api_key)).await.unwrap();
    Ok(url_result)
}

pub async fn provider_tmdb_meta_info_build(result_json: serde_json::Value) {
    let mut image_file_path = None;
    // create file path for poster
    image_file_path = image_path::meta_image_file_path("poster".to_string());
    let mut poster_file_path = None;
    if result_json["poster_path"] != None {
        image_file_path += result_json["poster_path"];
        if !Path::new(&image_file_path).exists() {
            if mk_lib_network::mk_download_file_from_url(
                "https://image.tmdb.org/t/p/original"
                    + result_json["poster_path"],
                image_file_path).await.unwrap() == false {
                // not found...so, none the image_file_path, which resets the poster_file_path
                image_file_path = None;
            }
        }
        poster_file_path = image_file_path;
    }
    // create file path for backdrop
    image_file_path = image_path::meta_image_file_path("backdrop".to_string());
    let mut backdrop_file_path = None;
    if result_json["backdrop_path"] != None {
        image_file_path += result_json["backdrop_path"].to_string();
        if !Path::new(&image_file_path).exists() {
            if mk_lib_network::mk_download_file_from_url(
                "https://image.tmdb.org/t/p/original"
                    + result_json["backdrop_path"],
                image_file_path).await.unwrap() == false {
                // not found...so, none the image_file_path, which resets the backdrop_file_path
                image_file_path = None;
            }
            backdrop_file_path = image_file_path;
        }
    }
    // set local image json
    if poster_file_path != None {
        poster_file_path = poster_file_path.replace("/mediakraken/web_app/static", "");
    }
    if backdrop_file_path != None {
        backdrop_file_path = backdrop_file_path.replace("/mediakraken/web_app/static", "");
    }
    let image_json = json!({
                            "Backdrop": backdrop_file_path,
                            "Poster": poster_file_path
                            });
    return (result_json["id"], image_json);
}

/*
    pub async fn com_tmdb_search(self, media_title, media_year=None, id_only=True,
                              media_type=common_global.DLMediaType.Movie.value):
        """
        # search for media title and year
        """
        if media_type == common_global.DLMediaType.Movie.value:
            async with httpx.AsyncClient() as client:
                search_json = await client.get('https://api.themoviedb.org/3/search/movie'
                                               '?api_key=%s&include_adult=1&query=%s'
                                               % (self.API_KEY, media_title.encode('utf-8')),
                                               timeout=3.05)
        else if media_type == common_global.DLMediaType.TV.value:
            async with httpx.AsyncClient() as client:
                search_json = await client.get('https://api.themoviedb.org/3/search/tv'
                                               '?api_key=%s&include_adult=1&query=%s'
                                               % (self.API_KEY, media_title.encode('utf-8')),
                                               timeout=3.05)
        else if media_type == common_global.DLMediaType.Person.value:
            async with httpx.AsyncClient() as client:
                search_json = await client.get('https://api.themoviedb.org/3/search/person'
                                               '?api_key=%s&include_adult=1&query=%s'
                                               % (self.API_KEY, media_title.encode('utf-8')),
                                               timeout=3.05)
        else:  // invalid search type
            return None, None
        // pull json since it's a coroutine above
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


pub async fn metadata_fetch_tmdb_person(db_connection, provider_name, download_data):
    // fetch and save json data via tmdb id
    result_json = await common_global.api_instance.com_tmdb_metadata_bio_by_id(
        download_data["mdq_provider_id"])
    if result_json == None or result_json.status_code == 502:
        asyncio.sleep(60)
        metadata_fetch_tmdb_person(db_connection, provider_name, download_data)
    else if result_json.status_code == 200:
        db_connection.db_meta_person_update(provider_name=provider_name,
                                                  provider_uuid=download_data['mdq_provider_id'],
                                                  person_bio=result_json.json(),
                                                  person_image=await common_global.api_instance.com_tmdb_meta_bio_image_build(
                                                      result_json.json()))
        db_connection.db_download_delete(download_data['mdq_id'])
        db_connection.db_commit()

 */