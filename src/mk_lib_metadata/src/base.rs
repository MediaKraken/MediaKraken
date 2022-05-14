#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;
use sqlx::types::Uuid;

#[path = "anime.rs"]
mod metadata_anime;
#[path = "book.rs"]
mod metadata_book;
#[path = "game.rs"]
mod metadata_game;
#[path = "movie.rs"]
mod metadata_movie;
#[path = "music.rs"]
mod metadata_music;
#[path = "music_video.rs"]
mod metadata_music_video;
#[path = "sports.rs"]
mod metadata_sports;
#[path = "tv.rs"]
mod metadata_tv;

#[path = "provider/anidb.rs"]
mod provider_anidb;
#[path = "provider/chart_lyrics.rs"]
mod provider_chart_lyrics;
#[path = "provider/imvdb.rs"]
mod provider_imvdb;
#[path = "provider/isbndb.rs"]
mod provider_isbndb;
#[path = "provider/musicbrainz.rs"]
mod provider_musicbrainz;
#[path = "provider/televisiontunes.rs"]
mod provider_televisiontunes;

#[path = "../mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;

pub async fn metadata_process(pool: &sqlx::PgPool,
                              provider_name: String,
                              download_data: serde_json::Value)
                              -> Result<(), Box<dyn Error>> {
    // TODO art, posters, trailers, etc in here as well
    if download_data["mdq_status"] == "Search" {
        metadata_search(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "Update" {
        metadata_update(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "Fetch" {
        metadata_fetch(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "FetchCastCrew" {
        metadata_castcrew(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "FetchReview" {
        metadata_review(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "FetchImage" {
        metadata_image(&pool, provider_name, download_data).await;
    } else if download_data["mdq_status"] == "FetchCollection" {
        metadata_collection(&pool, provider_name, download_data).await;
    }
    Ok(())
}

pub async fn metadata_update(pool: &sqlx::PgPool,
                             provider_name: String,
                             download_data: serde_json::Value)
                             -> Result<(), Box<dyn Error>> {
    // TODO horribly broken.  Need to add the dlid, that to update, etc
    Ok(())
}

pub async fn metadata_search(pool: &sqlx::PgPool,
                             provider_name: String,
                             download_data: serde_json::Value)
                             -> Result<(), sqlx::Error> {
    let mut metadata_uuid: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    let mut match_result = None;
    let mut set_fetch: bool = false;
    let mut lookup_halt: bool = false;
    let mut update_provider = String::new();
    if provider_name == "anidb" {
        metadata_uuid = metadata_anime::metadata_anime_lookup(&pool,
                                                              download_data,
                                                              download_data["Path"].to_string()).await.unwrap()["title"].to_string();
        if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            if match_result == None {
                // do lookup halt as we'll start all movies in tmdb
                lookup_halt = true;
            } else {
                set_fetch = true;
            }
        }
    } else if provider_name == "chart_lyrics" {
        //provider_chart_lyrics::provider_chart_lyrics_fetch(&pool, artist_name, song_name);
        lookup_halt = true;
    } else if provider_name == "comicvine" {
        lookup_halt = true;
    } else if provider_name == "giantbomb" {
        lookup_halt = true;
    } else if provider_name == "imdb" {
        lookup_halt = true;
    } else if provider_name == "imvdb" {
        (metadata_uuid, match_result) = metadata_music_video::metadata_music_video_lookup(
            &pool, download_data["mdq_path"].to_string()).await.unwrap();
        if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            if match_result == None {
                update_provider = "theaudiodb".to_string();
            } else {
                set_fetch = true;
            }
        }
    } else if provider_name == "isbndb" {
        (metadata_uuid, match_result) = provider_isbndb::metadata_book_search_isbndb(
            &pool, download_data["mdq_provider_id"].to_string()).await.unwrap();
        if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            lookup_halt = true;
        }
    } else if provider_name == "lastfm" {
        lookup_halt = true;
    } else if provider_name == "musicbrainz" {
        (metadata_uuid, match_result) = metadata_music::metadata_music_lookup(&pool,
                                                                              download_data).await.unwrap();
        if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            lookup_halt = true;
        }
    } else if provider_name == "omdb" {
        lookup_halt = true;
    } else if provider_name == "openlibrary" {
        lookup_halt = true;
    } else if provider_name == "pitchfork" {
        lookup_halt = true;
    } else if provider_name == "pornhub" {
        lookup_halt = true;
    } else if provider_name == "televisiontunes" {
        // if download succeeds remove dl
        // TODO....handle list return for title?
        metadata_uuid = provider_televisiontunes::provider_televisiontunes_theme_fetch(
            download_data["Path"]["title"].to_string(), "TODO Fake Path".to_string()).await.unwrap();
        if metadata_uuid != Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            // TODO add theme.mp3 dl"d above to media table
            mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(&pool, download_data["mdq_id"]).await.unwrap();
            Ok(());  // since it"s a search/fetch/insert in one shot
        } else {
            lookup_halt = true;
        }
    } else if provider_name == "theaudiodb" {
        lookup_halt = true;
    } else if provider_name == "thegamesdb" {
        lookup_halt = true;
    } else if provider_name == "thesportsdb" {
        (metadata_uuid, match_result) = metadata_sports::metadata_sports_lookup(&pool,
                                                                                download_data).await.unwrap();
        if metadata_uuid == Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() {
            if match_result == None {
                update_provider = "themoviedb".to_string();
            } else {
                set_fetch = true;
            }
        }
    } else if provider_name == "tv_intros" {
        lookup_halt = true;
    } else if provider_name == "twitch" {
        lookup_halt = true;
    }
    Ok(())
}

/*
    else if provider_name == "themoviedb":
        if download_data["mdq_que_type"] == common_global.DLMediaType.Movie.value:
            metadata_uuid, match_result = await metadata_provider_themoviedb.movie_search_tmdb(
                db_connection,
                download_data["mdq_path"]);
            // if match_result is an int, that means the lookup found a match but isn"t in db
            if metadata_uuid == None and type(match_result) != int:
                lookup_halt = true;
            else if metadata_uuid != None:
                    set_fetch = true;
        else if download_data["mdq_que_type"] == common_global.DLMediaType.TV.value:
            metadata_uuid, match_result = await metadata_tv.metadata_tv_lookup(db_connection,
                                                                               download_data[
                                                                                   "mdq_path"]);
            // if match_result is an int, that means the lookup found a match but isn"t in db
            if metadata_uuid == None and type(match_result) != int:
                lookup_halt = true;
            else if metadata_uuid != None:
                    set_fetch = true;
        else:
            // this will hit from type 0"s (trailers, etc)
            if metadata_uuid == None:
                lookup_halt = true;
            else if metadata_uuid != None:
                    set_fetch = true;


    // if search is being updated to new provider
    if update_provider != None:
        await db_connection.db_download_update_provider(update_provider, download_data["mdq_id"])
        await db_connection.db_commit()
        return metadata_uuid  # no need to continue with checks
    // if lookup halt set to ZZ so it doesn't get picked up by metadata dl queue
    if lookup_halt == true:
        await db_connection.db_download_update_provider("ZZ", download_data["mdq_id"]);
        await db_connection.db_commit();
        return metadata_uuid  # no need to continue with checks
    // if set fetch, set provider id and status on dl record
    if set_fetch == true:
        // first verify a download queue record doesn't exist for this id
        metadata_uuid = await db_connection.db_download_que_exists(download_data["mdq_id"],
                                                                   provider_name, str(match_result));
        if metadata_uuid == None:
            metadata_uuid = download_data["mdq_new_uuid"];
            await db_connection.db_update_media_id(download_data["mdq_provider_id"],
                                                   metadata_uuid);
            await db_connection.db_download_update(guid=download_data["mdq_id"],
                                                   status="Fetch",
                                                   provider_guid=match_result);
            await db_connection.db_commit();
    return metadata_uuid
*/

pub async fn metadata_fetch(pool: &sqlx::PgPool,
                            provider_name: String,
                            download_data: serde_json::Value)
                            -> Result<(), Box<dyn Error>> {
    if provider_name == "imvdb" {
        let imvdb_id = provider_imvdb::meta_fetch_save_imvdb(pool,
                                                             download_data["mdq_provider_id"].as_i64(),
                                                             Uuid::parse_str(&download_data["mdq_new_uuid"].to_string()).unwrap()).await.unwrap();
    }
    Ok(())
}
/*
    else if provider_name == "themoviedb":
        if download_data["mdq_que_type"] == common_global.DLMediaType.Person.value:
            metadata_provider_themoviedb.metadata_fetch_tmdb_person(
                db_connection, provider_name, download_data);
        else if download_data["mdq_que_type"] == common_global.DLMediaType.Movie.value:
            // removing the imdb check.....as com_tmdb_metadata_by_id converts it
            metadata_provider_themoviedb.movie_fetch_save_tmdb(db_connection,
                                                                     download_data[
                                                                         "mdq_provider_id"],
                                                                     download_data[
                                                                         "mdq_new_uuid"]);
        else if download_data["mdq_que_type"] == common_global.DLMediaType.TV.value:
            metadata_tv_tmdb.tv_fetch_save_tmdb(db_connection,
                                                download_data["mdq_provider_id"],
                                                download_data["mdq_new_uuid"]);
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();
*/

pub async fn metadata_castcrew(pool: &sqlx::PgPool,
                               provider_name: String,
                               download_data: serde_json::Value)
                               -> Result<(), Box<dyn Error>> {
    Ok(())
}

/*

pub async fn metadata_castcrew(db_connection, provider_name, download_data):
    """
    Fetch cast/crew from specified provider
    """
    // removed themoviedb call as it should be done during the initial fetch
    // setup for FetchReview
    db_connection.db_download_update(guid=download_data["mdq_id"],
                                           status="FetchReview");
    db_connection.db_commit();
*/

pub async fn metadata_image(pool: &sqlx::PgPool,
                            provider_name: String,
                            download_data: serde_json::Value)
                            -> Result<(), Box<dyn Error>> {
    Ok(())
}

/*
pub async fn metadata_image(db_connection, provider_name, download_data):
    """
    Fetch image from specified provider
    """
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();
*/


pub async fn metadata_review(pool: &sqlx::PgPool,
                             provider_name: String,
                             download_data: serde_json::Value)
                             -> Result<(), Box<dyn Error>> {
    Ok(())
}


/*
pub async fn metadata_review(db_connection, provider_name, download_data):
    """
    Fetch reviews from specified provider
    """
    if provider_name == "themoviedb" {
        metadata_provider_themoviedb.movie_fetch_save_tmdb_review(db_connection,
                                                                        download_data[
                                                                            "mdq_provider_id"]);
                                                                            }
    // review is last.....so can delete download que
    db_connection.db_download_delete(download_data["mdq_id"]);
    db_connection.db_commit();
*/

pub async fn metadata_collection(pool: &sqlx::PgPool,
                                 provider_name: String,
                                 download_data: serde_json::Value)
                                 -> Result<(), Box<dyn Error>> {
    Ok(())
}


/*
pub async fn metadata_collection(db_connection, provider_name, download_data):
    """
    Fetch collection from specified provider
    """
    if provider_name == "themoviedb" {
        metadata_provider_themoviedb.movie_fetch_save_tmdb_collection(db_connection,
                                                                            download_data[
                                                                                "mdq_provider_id"],
                                                                            download_data);
                                                                            }
    // only one record for this so nuke it
    db_connection.db_download_delete(download_data["mdq_id"]);
    db_connection.db_commit();

 */
