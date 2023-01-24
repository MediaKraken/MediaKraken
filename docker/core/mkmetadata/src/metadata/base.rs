#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::types::Uuid;
use std::error::Error;
use stdext::function_name;
use serde_json::json;
use torrent_name_parser::Metadata;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "adult.rs"]
mod metadata_adult;
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
#[path = "provider/tmdb.rs"]
mod provider_tmdb;

#[path = "guessit.rs"]
mod metadata_guessit;

#[path = "../mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

#[path = "../mk_lib_database_metadata_download_queue.rs"]
mod mk_lib_database_metadata_download_queue;
use crate::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

pub async fn metadata_process(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO art, posters, trailers, etc in here as well
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "metadata_process status": download_data.mm_download_status,  "provider": provider_name, "id": download_data.mm_download_provider_id }),
            )
            .await.unwrap();
    }
    if download_data.mm_download_status == "Search" {
        metadata_search(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "Update" {
        metadata_update(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "Fetch" {
        metadata_fetch(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "FetchCastCrew" {
        metadata_castcrew(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "FetchReview" {
        metadata_review(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "FetchImage" {
        metadata_image(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    } else if download_data.mm_download_status == "FetchCollection" {
        metadata_collection(&sqlx_pool, provider_name, download_data, provider_api_key).await;
    }
    Ok(())
}

pub async fn metadata_update(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO horribly broken.  Need to add the dlid, that to update, etc
    Ok(())
}

pub async fn metadata_search(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut metadata_uuid: Uuid = uuid::Uuid::nil();
    let mut set_fetch: bool = false;
    let mut lookup_halt: bool = false;
    let mut update_provider = String::new();
    let mut guessit_data: Metadata;
    if provider_name == "anidb" {
        (metadata_uuid, guessit_data) =
            metadata_guessit::metadata_guessit(&sqlx_pool, &download_data)
                .await
                .unwrap();
        if metadata_uuid == uuid::Uuid::nil() {
            metadata_uuid =
                metadata_anime::metadata_anime_lookup(&sqlx_pool, &download_data, guessit_data)
                    .await
                    .unwrap();
            // if metadata_uuid == uuid::Uuid::nil() {
            //     if match_result == None {
            //         // do lookup halt as we'll start all movies in tmdb
            //         lookup_halt = true;
            //     } else {
            //         set_fetch = true;
            //     }
            // }
        }
    } else if provider_name == "chart_lyrics" {
        //provider_chart_lyrics::provider_chart_lyrics_fetch(&sqlx_pool, artist_name, song_name);
        lookup_halt = true;
    } else if provider_name == "comicvine" {
        lookup_halt = true;
    } else if provider_name == "giantbomb" {
        lookup_halt = true;
    } else if provider_name == "imdb" {
        lookup_halt = true;
    } else if provider_name == "imvdb" {
        metadata_uuid =
            metadata_music_video::metadata_music_video_lookup(&sqlx_pool, &download_data)
                .await
                .unwrap();
        // if metadata_uuid == uuid::Uuid::nil() {
        //     if match_result == None {
        //         update_provider = "theaudiodb".to_string();
        //     } else {
        //         set_fetch = true;
        //     }
        // }
    } else if provider_name == "isbndb" {
        // metadata_uuid = provider_isbndb::metadata_book_search_isbndb(&sqlx_pool, download_data)
        //     .await
        //     .unwrap();
        if metadata_uuid == uuid::Uuid::nil() {
            lookup_halt = true;
        }
    } else if provider_name == "lastfm" {
        lookup_halt = true;
    } else if provider_name == "musicbrainz" {
        metadata_uuid = metadata_music::metadata_music_lookup(&sqlx_pool, &download_data)
            .await
            .unwrap();
        if metadata_uuid == uuid::Uuid::nil() {
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
            download_data.mm_download_path.unwrap(),
            "TODO Fake Path".to_string(),
        )
        .await
        .unwrap();
        if metadata_uuid != uuid::Uuid::nil() {
            // TODO add theme.mp3 dl"d above to media table
            mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
                &sqlx_pool,
                download_data.mm_download_guid,
            )
            .await
            .unwrap();
        } else {
            lookup_halt = true;
        }
    } else if provider_name == "theaudiodb" {
        lookup_halt = true;
    } else if provider_name == "thegamesdb" {
        lookup_halt = true;
    } else if provider_name == "themoviedb" {
        (metadata_uuid, guessit_data) =
            metadata_guessit::metadata_guessit(&sqlx_pool, &download_data)
                .await
                .unwrap();
        if download_data.mm_download_que_type == mk_lib_common_enum_media_type::DLMediaType::MOVIE {
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid =
                    metadata_movie::metadata_movie_lookup(&sqlx_pool, &download_data, guessit_data)
                        .await
                        .unwrap();
                // (metadata_uuid, match_result) = metadata_provider_themoviedb.movie_search_tmdb(
                //     &sqlx_pool,
                //     download_data);
                // // if match_result is an int, that means the lookup found a match but isn't in db
                // if metadata_uuid == uuid::Uuid::nil() && type(match_result) != int {
                //     lookup_halt = true;
                // }
                // else if metadata_uuid != uuid::Uuid::nil() {
                //         set_fetch = true;
                // }
            }
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::TV
        {
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid =
                    metadata_tv::metadata_tv_lookup(&sqlx_pool, &download_data, guessit_data)
                        .await
                        .unwrap();
                // (metadata_uuid, match_result) = metadata_tv.metadata_tv_lookup(&sqlx_pool, download_data);
                // // if match_result is an int, that means the lookup found a match but isn"t in db
                // if metadata_uuid == uuid::Uuid::nil() && type(match_result) != int {
                //     lookup_halt = true;
                // }
                // else if metadata_uuid != uuid::Uuid::nil() {
                //         set_fetch = true;
                // }
            }
        } else {
            // this will hit from type 0's (trailers, etc)
            if metadata_uuid == uuid::Uuid::nil() {
                lookup_halt = true;
            } else if metadata_uuid != uuid::Uuid::nil() {
                set_fetch = true;
            }
        }
    } else if provider_name == "thesportsdb" {
        metadata_uuid = metadata_sports::metadata_sports_lookup(&sqlx_pool, &download_data)
            .await
            .unwrap();
        // if metadata_uuid == uuid::Uuid::nil() {
        //     if match_result == None {
        //         update_provider = "themoviedb".to_string();
        //     } else {
        //         set_fetch = true;
        //     }
        // }
    } else if provider_name == "tv_intros" {
        lookup_halt = true;
    } else if provider_name == "twitch" {
        lookup_halt = true;
    }
    Ok(())
}

/*

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

pub async fn metadata_fetch(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    if provider_name == "imvdb" {
        let imvdb_id = provider_imvdb::meta_fetch_save_imvdb(
            sqlx_pool,
            download_data.mm_download_provider_id,
            download_data.mm_download_new_uuid,
        )
        .await
        .unwrap();
    } else if provider_name == "themoviedb" {
        if download_data.mm_download_que_type == mk_lib_common_enum_media_type::DLMediaType::PERSON
        {
            provider_tmdb::provider_tmdb_person_fetch(
                sqlx_pool,
                download_data.mm_download_provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await;
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::MOVIE
        {
            // removing the imdb check.....as com_tmdb_metadata_by_id converts it
            provider_tmdb::provider_tmdb_movie_fetch(
                sqlx_pool,
                download_data.mm_download_provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await;
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::TV
        {
            provider_tmdb::provider_tmdb_tv_fetch(
                sqlx_pool,
                download_data.mm_download_provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await;
        }
    }
    mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

pub async fn metadata_castcrew(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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

pub async fn metadata_image(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // TODO grab the actual image
    mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

pub async fn metadata_review(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // review is last.....so can delete download que
    mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

pub async fn metadata_collection(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &String,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    // only one record for this so nuke it
    mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}
