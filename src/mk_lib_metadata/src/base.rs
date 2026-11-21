use crate::guessit;
use mk_lib_common::mk_lib_common_enum_media_type;
use mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;
use parse_torrent_title::ParsedTitle;
use serde_json::json;
use std::env;
use std::error::Error;

use super::adult as metadata_adult;
use super::anime as metadata_anime;
use super::book as metadata_book;
use super::game as metadata_game;
use super::movie as metadata_movie;
use super::music as metadata_music;
use super::music_video as metadata_music_video;
use super::person as metadata_person;
use super::sports as metadata_sports;
use super::tv as metadata_tv;

use super::metadata_provider::anidb as provider_anidb;
use super::metadata_provider::barcodespider as provider_barcodespider;
use super::metadata_provider::chart_lyrics as provider_chart_lyrics;
use super::metadata_provider::imvdb as provider_imvdb;
use super::metadata_provider::isbndb as provider_isbndb;
use super::metadata_provider::musicbrainz as provider_musicbrainz;
use super::metadata_provider::televisiontunes as provider_televisiontunes;
use super::metadata_provider::tmdb as provider_tmdb;
use super::metadata_provider::upcitemdb as provider_upcitemdb;

/// Pushes a diagnostic payload to Loki when `DEBUG=true`.
///
/// Mirrors the env gate mkmetadata uses for its own debug logging; push
/// failures are swallowed on purpose so a logging outage can never mask
/// a processing error.
pub(crate) async fn metadata_debug_log(payload: serde_json::Value) {
    if env::var("DEBUG").ok().as_deref() == Some("true") {
        let _ = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(payload).await;
    }
}

pub async fn metadata_process(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO art, posters, trailers, etc in here as well
    match download_data.mm_download_status.as_str() {
        "Search" => {
            metadata_search(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "Update" => {
            metadata_update(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "Fetch" => {
            metadata_fetch(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "FetchCastCrew" => {
            metadata_castcrew(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "FetchReview" => {
            metadata_review(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "FetchImage" => {
            metadata_image(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        "FetchCollection" => {
            metadata_collection(sqlx_pool, provider_name, download_data, provider_api_key).await?
        }
        _ => {}
    }

    Ok(())
}

pub async fn metadata_update(
    _sqlx_pool: &sqlx::PgPool,
    _provider_name: String,
    _download_data: DBDownloadQueueByProviderList,
    _provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO horribly broken.  Need to add the dlid, that to update, etc
    Ok(())
}

pub async fn metadata_search(
    sqlx_pool: &sqlx::PgPool,
    provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    let mut metadata_uuid: uuid::Uuid = uuid::Uuid::nil();
    let mut set_fetch: bool = false;
    let mut lookup_halt: bool = false;
    let mut matched_local_db: bool = false;
    let mut matched_tmdb_id: Option<i32> = None;
    let _update_provider = String::new();
    let guessit_data: ParsedTitle;
    metadata_debug_log(json!({
        "Type": "Search",
        "Module": std::module_path!(),
        "DL Guid": download_data.mm_download_guid,
        "QueType": download_data.mm_download_que_type,
        "Status": download_data.mm_download_status,
        "Provider": provider_name,
        "ID": download_data.mm_download_provider_id,
    }))
    .await;
    if provider_name == "anidb" {
        guessit_data = guessit::metadata_guessit(
            sqlx_pool,
            &download_data,
            "fake".to_string(),
            0,
            metadata_uuid,
        )
        .await?;
        if metadata_uuid == uuid::Uuid::nil() {
            metadata_uuid =
                metadata_anime::metadata_anime_lookup(sqlx_pool, &download_data, guessit_data)
                    .await?;
            // if metadata_uuid == uuid::Uuid::nil() {
            //     if match_result == None {
            //         // do lookup halt as we'll start all movies in tmdb
            //         lookup_halt = true;
            //     } else {
            //         set_fetch = true;
            //     }
            // }
        }
    } else if provider_name == "chart_lyrics"
        || provider_name == "comicvine"
        || provider_name == "giantbomb"
        || provider_name == "imdb"
    {
        //provider_chart_lyrics::provider_chart_lyrics_fetch(&sqlx_pool, artist_name, song_name);
        lookup_halt = true;
    } else if provider_name == "imvdb" {
        metadata_uuid =
            metadata_music_video::metadata_music_video_lookup(sqlx_pool, &download_data).await?;
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
        //     .map_err(|e| e)?;
        if metadata_uuid == uuid::Uuid::nil() {
            lookup_halt = true;
        }
    } else if provider_name == "lastfm" {
        lookup_halt = true;
    } else if provider_name == "musicbrainz" {
        metadata_uuid = metadata_music::metadata_music_lookup(sqlx_pool, &download_data).await?;
        if metadata_uuid == uuid::Uuid::nil() {
            lookup_halt = true;
        }
    } else if provider_name == "omdb"
        || provider_name == "openlibrary"
        || provider_name == "pitchfork"
        || provider_name == "pornhub"
    {
        lookup_halt = true;
    } else if provider_name == "televisiontunes" {
        // if download succeeds remove dl
        // TODO....handle list return for title?
        metadata_uuid = provider_televisiontunes::provider_televisiontunes_theme_fetch(
            download_data
                .mm_download_path
                .ok_or("missing download_path")?,
            "TODO Fake Path".to_string(),
        )
        .await?;
        if metadata_uuid != uuid::Uuid::nil() {
            // TODO add theme.mp3 dl'd above to media table
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
                sqlx_pool,
                download_data.mm_download_guid,
            )
            .await?;
        } else {
            lookup_halt = true;
        }
    } else if provider_name == "theaudiodb" || provider_name == "thegamesdb" {
        lookup_halt = true;
    } else if provider_name == "themoviedb" || provider_name == "Z" {
        // "Z" is the scanner's catch-all provider: it runs the
        // local-db search. A hit links the metadata guid into the
        // media record below; a movie/tv miss re-points the row at
        // "themoviedb" inside the lookups. Movie/tv rows already under
        // "themoviedb" got that local search on the Z pass (or are a
        // re-poll of it), so skip the re-lookup and go straight to the
        // provider API search below, which re-checks the db for the
        // returned id.
        guessit_data = guessit::metadata_guessit(
            sqlx_pool,
            &download_data,
            "fake".to_string(),
            0,
            metadata_uuid,
        )
        .await?;
        let run_local_lookup = provider_name == "Z"
            || !(download_data.mm_download_que_type
                == mk_lib_common_enum_media_type::DLMediaType::MOVIE
                || download_data.mm_download_que_type
                    == mk_lib_common_enum_media_type::DLMediaType::TV);
        if run_local_lookup {
            if download_data.mm_download_que_type
                == mk_lib_common_enum_media_type::DLMediaType::MOVIE
            {
                if metadata_uuid == uuid::Uuid::nil() {
                    metadata_uuid = metadata_movie::metadata_movie_lookup(
                        sqlx_pool,
                        &download_data,
                        guessit_data.clone(),
                    )
                    .await?;
                }
            } else if download_data.mm_download_que_type
                == mk_lib_common_enum_media_type::DLMediaType::TV
            {
                if metadata_uuid == uuid::Uuid::nil() {
                    metadata_uuid = metadata_tv::metadata_tv_lookup(
                        sqlx_pool,
                        &download_data,
                        guessit_data.clone(),
                    )
                    .await?;
                }
            } else if download_data.mm_download_que_type
                == mk_lib_common_enum_media_type::DLMediaType::PERSON
            {
                if metadata_uuid == uuid::Uuid::nil() {
                    metadata_uuid = metadata_person::metadata_person_lookup(
                        sqlx_pool,
                        &download_data,
                        guessit_data.clone(),
                    )
                    .await?;
                }
            } else {
                // this will hit from type 0's (trailers, etc)
                if metadata_uuid == uuid::Uuid::nil() {
                    lookup_halt = true;
                } else if metadata_uuid != uuid::Uuid::nil() {
                    set_fetch = true;
                }
            }
        }
        // The movie/tv lookups return mm_download_new_uuid as their
        // "not found" placeholder and the person lookup is a nil stub,
        // so a local hit is anything that is neither.
        matched_local_db = metadata_uuid != uuid::Uuid::nil()
            && metadata_uuid != download_data.mm_download_new_uuid;
        if matched_local_db {
            // The metadata record already exists: point the media row at
            // it and remove the queue row. Both run in one transaction,
            // so an error leaves the row for the next poll.
            mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
                sqlx_pool,
                download_data.mm_download_new_uuid,
                metadata_uuid,
                &download_data.mm_download_guid,
            )
            .await?;
            metadata_debug_log(json!({
                "Type": "MediaLinked",
                "Module": std::module_path!(),
                "Stage": "Search",
                "DL Guid": download_data.mm_download_guid,
                "MediaGuid": download_data.mm_download_new_uuid,
                "MetadataGuid": metadata_uuid,
            }))
            .await;
        } else if !lookup_halt {
            if provider_name == "Z" {
                if metadata_uuid == uuid::Uuid::nil() {
                    // The Z worker has no API key and the person lookup
                    // is a nil stub, so hand the row to the themoviedb
                    // worker, which runs the remote search below. Movie/tv
                    // misses already re-pointed their rows inside the
                    // lookup and returned the new_uuid placeholder.
                    metadata_debug_log(json!({
                        "Type": "SearchZRepoint",
                        "Module": std::module_path!(),
                        "DL Guid": download_data.mm_download_guid,
                        "QueType": download_data.mm_download_que_type,
                    }))
                    .await;
                    mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(
                        sqlx_pool,
                        "themoviedb".to_string(),
                        download_data.mm_download_guid,
                    )
                    .await?;
                }
            } else {
                // Local miss under the worker that holds the API key:
                // search TMDB and transition the row to "Fetch" with the
                // matched id. No match dead-letters the row to "ZZ" so
                // it is not re-polled (and re-searched) every cycle.
                match provider_tmdb::provider_tmdb_search(
                    guessit_data,
                    download_data.mm_download_que_type,
                    provider_api_key,
                )
                .await
                {
                    Some(tmdb_id) => {
                        // "DB record for ID returned?": if a metadata
                        // record for this id already exists, link the
                        // media row to it and delete the DL row instead
                        // of re-fetching. Without this check the Fetch
                        // stage would insert a second row for the same
                        // id. Tmdb ids for movies and tv share the same
                        // id space, so each type is checked against its
                        // own table only.
                        let mut existing_guid: Option<uuid::Uuid> = None;
                        if download_data.mm_download_que_type
                            == mk_lib_common_enum_media_type::DLMediaType::MOVIE
                        {
                            existing_guid =
                                mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_guid_by_tmdb(
                                    sqlx_pool,
                                    tmdb_id,
                                )
                                .await?;
                        } else if download_data.mm_download_que_type
                            == mk_lib_common_enum_media_type::DLMediaType::TV
                        {
                            existing_guid =
                                mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_tv_guid_by_tmdb(
                                    sqlx_pool,
                                    tmdb_id,
                                )
                                .await?;
                        }
                        if let Some(guid) = existing_guid {
                            metadata_uuid = guid;
                            matched_local_db = true;
                            metadata_debug_log(json!({
                                "Type": "SearchExistingRecord",
                                "Module": std::module_path!(),
                                "DL Guid": download_data.mm_download_guid,
                                "QueType": download_data.mm_download_que_type,
                                "TmdbId": tmdb_id,
                                "MetadataUuid": guid,
                            }))
                            .await;
                            mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
                                sqlx_pool,
                                download_data.mm_download_new_uuid,
                                guid,
                                &download_data.mm_download_guid,
                            )
                            .await?;
                        } else {
                            // If another row is already past "Search"
                            // for this id, leave the row queued and
                            // re-check next cycle once that row
                            // finishes.
                            let already_queued =
                                mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_exists(
                                    sqlx_pool,
                                    "themoviedb".to_string(),
                                    download_data.mm_download_que_type,
                                    tmdb_id,
                                )
                                .await?;
                            if already_queued {
                                metadata_debug_log(json!({
                                    "Type": "SearchDuplicate",
                                    "Module": std::module_path!(),
                                    "DL Guid": download_data.mm_download_guid,
                                    "QueType": download_data.mm_download_que_type,
                                    "TmdbId": tmdb_id,
                                }))
                                .await;
                            } else {
                                matched_tmdb_id = Some(tmdb_id);
                                set_fetch = true;
                            }
                        }
                    }
                    None => {
                        lookup_halt = true;
                    }
                }
            }
        }
        if set_fetch {
            let tmdb_id = matched_tmdb_id.ok_or("set_fetch without a matched tmdb id")?;
            // provider + id + status in one UPDATE so a poll loop can
            // never observe an intermediate state.
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_set_fetch(
                sqlx_pool,
                download_data.mm_download_guid,
                provider_name.clone(),
                tmdb_id,
            )
            .await?;
            metadata_debug_log(json!({
                "Type": "SetFetch",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "Provider": provider_name,
                "TmdbId": tmdb_id,
            }))
            .await;
        } else if lookup_halt {
            // Dead-letter the row so unresolvable rows stop being
            // re-polled every cycle. Only this branch sets lookup_halt
            // at this point; the other providers keep their existing
            // "row left in queue" behavior.
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_metadata_download_queue_update_provider(
                sqlx_pool,
                "ZZ".to_string(),
                download_data.mm_download_guid,
            )
            .await?;
            metadata_debug_log(json!({
                "Type": "DeadLetter",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "QueType": download_data.mm_download_que_type,
                "Provider": provider_name,
            }))
            .await;
        }
    } else if provider_name == "thesportsdb" {
        metadata_uuid = metadata_sports::metadata_sports_lookup(sqlx_pool, &download_data).await?;
        // if metadata_uuid == uuid::Uuid::nil() {
        //     if match_result == None {
        //         update_provider = "themoviedb".to_string();
        //     } else {
        //         set_fetch = true;
        //     }
        // }
    } else if provider_name == "tv_intros" || provider_name == "twitch" {
        lookup_halt = true;
    }
    metadata_debug_log(json!({
        "Type": "SearchResult",
        "Module": std::module_path!(),
        "DL Guid": download_data.mm_download_guid,
        "Provider": provider_name,
        "MetadataUuid": metadata_uuid,
        "MatchedLocalDb": matched_local_db,
        "TmdbId": matched_tmdb_id,
        "SetFetch": set_fetch,
        "LookupHalt": lookup_halt,
    }))
    .await;
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
    provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // True when a themoviedb fetch inserted the metadata record; the
    // media row is linked to it below only in that case.
    let mut fetched: bool = false;
    // if provider_name == "barcodespider" {
    //     let _barcode_id = provider_barcodespider::provider_barcodespider_fetch_by_upc(
    //         sqlx_pool,
    //         &download_data.mm_download_provider_id.unwrap(),
    //         &provider_api_key,
    //     )
    //     .await
    //     .map_err(|e| e)?;
    // } else
    if provider_name == "imvdb" {
        let _imvdb_id = provider_imvdb::provider_imvdb_video_fetch_by_id(
            sqlx_pool,
            download_data
                .mm_download_provider_id
                .ok_or("missing provider_id")?,
            download_data.mm_download_new_uuid,
            provider_api_key,
        )
        .await?;
    } else if provider_name == "themoviedb" {
        if download_data.mm_download_que_type == mk_lib_common_enum_media_type::DLMediaType::PERSON
        {
            if env::var("DEBUG").ok().as_deref() == Some("true") {
                let _ = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
                    json!({ "Type": "Person", "Module": std::module_path!(), "DL Guid": download_data.mm_download_guid, "Status": download_data.mm_download_status, "Provider": "themoviedb", "ID": download_data.mm_download_provider_id }),
                )
                .await;
            }
            let provider_id = download_data
                .mm_download_provider_id
                .ok_or("missing provider_id")?;
            fetched = match provider_tmdb::provider_tmdb_person_fetch(
                sqlx_pool,
                provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await
            {
                Ok(inserted) => inserted,
                Err(e) => {
                    eprintln!("TMDB person fetch failed: {e}");
                    false
                }
            };
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::MOVIE
        {
            // removing the imdb check.....as com_tmdb_metadata_by_id converts it
            let provider_id = download_data
                .mm_download_provider_id
                .ok_or("missing provider_id")?;
            metadata_debug_log(json!({
                "Type": "FetchMovie",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "Provider": "themoviedb",
                "TmdbId": provider_id,
            }))
            .await;
            // The match completes before any await: the Err binding is a
            // Box<dyn Error> (!Send) whose scope must not span an await
            // point, so the error is materialized to a String in-arm.
            let (inserted, fetch_err) = match provider_tmdb::provider_tmdb_movie_fetch(
                sqlx_pool,
                provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await
            {
                Ok(inserted) => (inserted, None),
                Err(e) => (false, Some(e.to_string())),
            };
            if let Some(err) = fetch_err {
                eprintln!("TMDB movie fetch failed: {err}");
                metadata_debug_log(json!({
                    "Type": "FetchMovieError",
                    "Module": std::module_path!(),
                    "DL Guid": download_data.mm_download_guid,
                    "TmdbId": provider_id,
                    "Error": err,
                }))
                .await;
            }
            fetched = inserted;
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::TV
        {
            let provider_id = download_data
                .mm_download_provider_id
                .ok_or("missing provider_id")?;
            metadata_debug_log(json!({
                "Type": "FetchTv",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "Provider": "themoviedb",
                "TmdbId": provider_id,
            }))
            .await;
            // See the movie arm above for why the match must complete
            // before any await.
            let (inserted, fetch_err) = match provider_tmdb::provider_tmdb_tv_fetch(
                sqlx_pool,
                provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await
            {
                Ok(inserted) => (inserted, None),
                Err(e) => (false, Some(e.to_string())),
            };
            if let Some(err) = fetch_err {
                eprintln!("TMDB TV fetch failed: {err}");
                metadata_debug_log(json!({
                    "Type": "FetchTvError",
                    "Module": std::module_path!(),
                    "DL Guid": download_data.mm_download_guid,
                    "TmdbId": provider_id,
                    "Error": err,
                }))
                .await;
            }
            fetched = inserted;
        } else if download_data.mm_download_que_type
            == mk_lib_common_enum_media_type::DLMediaType::COLLECTION
        {
            let provider_id = download_data
                .mm_download_provider_id
                .ok_or("missing provider_id")?;
            metadata_debug_log(json!({
                "Type": "FetchCollection",
                "Module": std::module_path!(),
                "DL Guid": download_data.mm_download_guid,
                "Provider": "themoviedb",
                "TmdbId": provider_id,
            }))
            .await;
            // See the movie arm above for why the match must complete
            // before any await.
            let (inserted, fetch_err) = match provider_tmdb::provider_tmdb_collection_fetch(
                sqlx_pool,
                provider_id,
                download_data.mm_download_new_uuid,
                provider_api_key,
            )
            .await
            {
                Ok(inserted) => (inserted, None),
                Err(e) => (false, Some(e.to_string())),
            };
            if let Some(err) = fetch_err {
                eprintln!("TMDB collection fetch failed: {err}");
                metadata_debug_log(json!({
                    "Type": "FetchCollectionError",
                    "Module": std::module_path!(),
                    "DL Guid": download_data.mm_download_guid,
                    "TmdbId": provider_id,
                    "Error": err,
                }))
                .await;
            }
            fetched = inserted;
        }
    }
    //         sqlx_pool,
    //         vec![&download_data.mm_download_provider_id.unwrap()],
    //         &provider_api_key,
    //     )
    //     .await
    //     .map_err(|e| e)?;
    // }
    if fetched {
        // The fetch inserted the metadata record using
        // mm_download_new_uuid as its guid — the same uuid the scanner
        // stored as the media row's guid — so point the media row at it.
        // Passing the DL guid makes the link and the queue-row delete
        // run in one transaction.
        mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_update_metadata_guid(
            sqlx_pool,
            download_data.mm_download_new_uuid,
            download_data.mm_download_new_uuid,
            &download_data.mm_download_guid,
        )
        .await?;
        metadata_debug_log(json!({
            "Type": "MediaLinked",
            "Module": std::module_path!(),
            "Stage": "Fetch",
            "DL Guid": download_data.mm_download_guid,
            "MediaGuid": download_data.mm_download_new_uuid,
            "MetadataGuid": download_data.mm_download_new_uuid,
        }))
        .await;
    } else if provider_name == "themoviedb" {
        // The fetch did not insert a metadata record (TMDB error status
        // or API failure): leave the queue row in place so the next
        // poll retries the fetch instead of dropping the row and
        // leaving the media unmatched.
        metadata_debug_log(json!({
            "Type": "FetchSkipped",
            "Module": std::module_path!(),
            "Stage": "Fetch",
            "DL Guid": download_data.mm_download_guid,
            "MediaGuid": download_data.mm_download_new_uuid,
        }))
        .await;
    } else {
        // Other providers don't track fetch success here, so keep the
        // consume-once behavior.
        let _result =
            mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
                sqlx_pool,
                download_data.mm_download_guid,
            )
            .await;
    }
    Ok(())
}

pub async fn metadata_castcrew(
    _sqlx_pool: &sqlx::PgPool,
    _provider_name: String,
    _download_data: DBDownloadQueueByProviderList,
    _provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
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
    _provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    _provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO grab the actual image
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

pub async fn metadata_review(
    sqlx_pool: &sqlx::PgPool,
    _provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    _provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // review is last.....so can delete download que
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

pub async fn metadata_collection(
    sqlx_pool: &sqlx::PgPool,
    _provider_name: String,
    download_data: DBDownloadQueueByProviderList,
    _provider_api_key: &str,
) -> Result<(), Box<dyn Error>> {
    // only one record for this so nuke it
    let _result = mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::mk_lib_database_download_queue_delete(
        sqlx_pool,
        download_data.mm_download_guid,
    )
    .await;
    Ok(())
}

#[cfg(test)]
mod send_tests {
    use super::*;

    fn assert_send_future<F: std::future::Future + Send>(_: F) {}

    fn dummy_queue_row() -> DBDownloadQueueByProviderList {
        DBDownloadQueueByProviderList {
            mm_download_guid: uuid::Uuid::nil(),
            mm_download_que_type: 0,
            mm_download_new_uuid: uuid::Uuid::nil(),
            mm_download_provider_id: None,
            mm_download_status: String::from("Search"),
            mm_download_path: None,
        }
    }

    /// The mkmetadata workers `tokio::spawn` these loops, which requires
    /// the futures to be `Send`. Assert it at compile time here so a
    /// non-Send value held across an await point (e.g. a `Box<dyn Error>`)
    /// breaks this crate's build instead of only breaking mkmetadata's.
    ///
    /// The futures are never polled; the pool only satisfies the
    /// signatures. sqlx 0.9's `connect_lazy` still needs a Tokio context.
    #[test]
    fn metadata_futures_are_send() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let pool = rt
            .block_on(async { sqlx::PgPool::connect_lazy("postgres://user:pass@localhost/db") })
            .unwrap();
        assert_send_future(metadata_process(
            &pool,
            String::from("themoviedb"),
            dummy_queue_row(),
            "",
        ));
        assert_send_future(metadata_search(
            &pool,
            String::from("Z"),
            dummy_queue_row(),
            "",
        ));
        assert_send_future(metadata_fetch(
            &pool,
            String::from("themoviedb"),
            dummy_queue_row(),
            "",
        ));
    }
}
