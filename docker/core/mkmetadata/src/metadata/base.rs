use sqlx::types::Uuid;

#[path = "anime.rs"]
mod mk_anime;

#[path = "book.rs"]
mod mk_book;

#[path = "game.rs"]
mod mk_game;

#[path = "movie.rs"]
mod mk_movie;

#[path = "music.rs"]
mod mk_music;

#[path = "music_video.rs"]
mod mk_music_video;

#[path = "sports.rs"]
mod mk_sports;

#[path = "tv.rs"]
mod mk_tv;

pub async fn metadata_process(pool: &sqlx::PgPool,
                              provider_name: String,
                              download_data: serde_json::Value) {
    // TODO art, posters, trailers, etc in here as well
    if download_data["mdq_status"] == "Search" {
        metadata_search(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "Update" {
        metadata_update(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "Fetch" {
        metadata_fetch(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "FetchCastCrew" {
        metadata_castcrew(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "FetchReview" {
        metadata_review(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "FetchImage" {
        metadata_image(&pool, provider_name, download_data);
    } else if download_data["mdq_status"] == "FetchCollection" {
        metadata_collection(&pool, provider_name, download_data);
    }
}

pub async fn metadata_update(pool: &sqlx::PgPool,
                             provider_name: String,
                             download_data: serde_json::Value) {
    // TODO horribly broken.  Need to add the dlid, that to update, etc
}

pub async fn metadata_search(pool: &sqlx::PgPool,
                             provider_name: String,
                             download_data: serde_json::Value) {
    let mut metadata_uuid: Uuid = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;
    let mut match_result = None;
    let mut set_fetch = false;
    let mut lookup_halt = false;
    let mut update_provider = None;
    if provider_name == "anidb" {
        metadata_uuid = mk_anime::metadata_anime_lookup(&pool,
                                                             download_data,
                                                             guessit(download_data["Path"])["title"]);
        if metadata_uuid == None {
            if match_result == None {
                // do lookup halt as we'll start all movies in tmdb
                lookup_halt = true;
            } else {
                set_fetch = true;
            }
        }
    } else if provider_name == "chart_lyrics" {
        common_metadata_provider_chart_lyrics.com_meta_chart_lyrics(artist_name, song_name);
        lookup_halt = true;
    } else if provider_name == "comicvine" {
        lookup_halt = true;
    } else if provider_name == "discogs" {
        lookup_halt = true;
    } else if provider_name == "giantbomb" {
        lookup_halt = true;
    } else if provider_name == "imdb" {
        lookup_halt = true;
    } else if provider_name == "imvdb" {
        metadata_uuid, match_result = metadata_music_video.metadata_music_video_lookup(
            db_connection,
            download_data["mdq_path"]);
        if metadata_uuid == None {
            if match_result == None {
                update_provider = "theaudiodb";
            } else {
                set_fetch = true;
            }
        }
    } else if provider_name == "isbndb" {
        metadata_uuid, match_result = metadata_provider_isbndb.metadata_periodicals_search_isbndb(
            db_connection, download_data["mdq_provider_id"]);
        if metadata_uuid == None {
            lookup_halt = true;
        }
    } else if provider_name == "lastfm" {
        lookup_halt = true;
    } else if provider_name == "musicbrainz" {
        metadata_uuid, match_result = metadata_music.metadata_music_lookup(db_connection,
                                             download_data);
        if metadata_uuid == None {
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
        metadata_uuid = common_metadata_tv_theme.com_tvtheme_download(
            guessit(download_data["Path"])["title"]);
        if metadata_uuid != None {
            // TODO add theme.mp3 dl"d above to media table
            db_connection.db_download_delete(download_data["mdq_id"]);
            db_connection.db_commit();
            return;  // since it"s a search/fetch/insert in one shot
        } else {
            lookup_halt = true;
        }
    } else if provider_name == "theaudiodb" {
        lookup_halt = true;
    } else if provider_name == "thegamesdb" {
        lookup_halt = true;
    }
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
    else if provider_name == "thesportsdb":
        metadata_uuid, match_result = await metadata_sports.metadata_sports_lookup(db_connection,
                                                                                   download_data);
        if metadata_uuid == None:
            if match_result == None:
                update_provider = "themoviedb";
            else:
                set_fetch = true;
    else if provider_name == "tv_intros" {
        lookup_halt = true;
        }
    else if provider_name == "twitch" {
        lookup_halt = true;
        }

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


async def metadata_fetch(db_connection, provider_name, download_data):
    """
    Fetch main metadata for specified provider
    """
    if provider_name == "imvdb":
        imvdb_id = await metadata_provider_imvdb.movie_fetch_save_imvdb(db_connection,
                                                                        download_data[
                                                                            "mdq_provider_id"],
                                                                        download_data[
                                                                            "mdq_new_uuid"]);
    else if provider_name == "themoviedb":
        if download_data["mdq_que_type"] == common_global.DLMediaType.Person.value:
            await metadata_provider_themoviedb.metadata_fetch_tmdb_person(
                db_connection, provider_name, download_data);
        else if download_data["mdq_que_type"] == common_global.DLMediaType.Movie.value:
            // removing the imdb check.....as com_tmdb_metadata_by_id converts it
            await metadata_provider_themoviedb.movie_fetch_save_tmdb(db_connection,
                                                                     download_data[
                                                                         "mdq_provider_id"],
                                                                     download_data[
                                                                         "mdq_new_uuid"]);
        else if download_data["mdq_que_type"] == common_global.DLMediaType.TV.value:
            await metadata_tv_tmdb.tv_fetch_save_tmdb(db_connection,
                                                      download_data["mdq_provider_id"],
                                                      download_data["mdq_new_uuid"]);
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();


async def metadata_castcrew(db_connection, provider_name, download_data):
    """
    Fetch cast/crew from specified provider
    """
    // removed themoviedb call as it should be done during the initial fetch
    // setup for FetchReview
    await db_connection.db_download_update(guid=download_data["mdq_id"],
                                           status="FetchReview");
    await db_connection.db_commit();


async def metadata_image(db_connection, provider_name, download_data):
    """
    Fetch image from specified provider
    """
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();


async def metadata_review(db_connection, provider_name, download_data):
    """
    Fetch reviews from specified provider
    """
    if provider_name == "themoviedb" {
        await metadata_provider_themoviedb.movie_fetch_save_tmdb_review(db_connection,
                                                                        download_data[
                                                                            "mdq_provider_id"]);
                                                                            }
    // review is last.....so can delete download que
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();


async def metadata_collection(db_connection, provider_name, download_data):
    """
    Fetch collection from specified provider
    """
    if provider_name == "themoviedb" {
        await metadata_provider_themoviedb.movie_fetch_save_tmdb_collection(db_connection,
                                                                            download_data[
                                                                                "mdq_provider_id"],
                                                                            download_data);
                                                                            }
    // only one record for this so nuke it
    await db_connection.db_download_delete(download_data["mdq_id"]);
    await db_connection.db_commit();

 */
