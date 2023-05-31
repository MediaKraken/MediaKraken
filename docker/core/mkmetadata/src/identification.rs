use std::error::Error;
use torrent_name_parser::Metadata;
use crate::mk_lib_metadata;
use crate::mk_lib_common;
use crate::mk_lib_database::database_metadata::mk_lib_database_metadata_download_queue::DBDownloadQueueByProviderList;

pub async fn metadata_identification(
    sqlx_pool: &sqlx::PgPool,
    dl_row: &DBDownloadQueueByProviderList,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    let mut metadata_uuid: uuid::Uuid = uuid::Uuid::nil();
    let mut guessit_data: Metadata;
    match dl_row.mm_download_que_type {
        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::ADULT
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::ADULT_SCENE => {
            (metadata_uuid, guessit_data) = mk_lib_metadata::guessit::metadata_guessit(&sqlx_pool, &dl_row)
                .await
                .unwrap();
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid =
                mk_lib_metadata::adult::metadata_adult_lookup(&sqlx_pool, &dl_row, guessit_data)
                        .await
                        .unwrap();
            }
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::ANIME => {
            (metadata_uuid, guessit_data) = mk_lib_metadata::guessit::metadata_guessit(&sqlx_pool, &dl_row)
                .await
                .unwrap();
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid =
                mk_lib_metadata::anime::metadata_anime_lookup(&sqlx_pool, &dl_row, guessit_data)
                        .await
                        .unwrap();
            }
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_CHD
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_CINEMATICS
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_SPEEDRUN
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_SUPERPLAY
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_TRAILER
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_ISO
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::GAME_ROM => {
            metadata_uuid = mk_lib_metadata::game::metadata_game_lookup(&sqlx_pool, &dl_row)
                .await
                .unwrap();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_BOOK
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_COMIC
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_COMIC_STRIP
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_MAGAZINE
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_GRAPHIC_NOVEL => {
            metadata_uuid = mk_lib_metadata::book::metadata_book_lookup(&sqlx_pool, &dl_row)
                .await
                .unwrap();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER => {
            (metadata_uuid, guessit_data) = mk_lib_metadata::guessit::metadata_guessit(&sqlx_pool, &dl_row)
                .await
                .unwrap();
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid =
                mk_lib_metadata::movie::metadata_movie_lookup(&sqlx_pool, &dl_row, guessit_data)
                        .await
                        .unwrap();
            }
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::PICTURE => {
            metadata_uuid = uuid::Uuid::new_v4();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC_ALBUM
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC_LYRICS
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC_SONG => {
            metadata_uuid = mk_lib_metadata::music::metadata_music_lookup(&sqlx_pool, &dl_row)
                .await
                .unwrap();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::MUSIC_VIDEO => {
            metadata_uuid = mk_lib_metadata::music_video::metadata_music_video_lookup(&sqlx_pool, &dl_row)
                .await
                .unwrap();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::SPORTS => {
            metadata_uuid = mk_lib_metadata::sports::metadata_sports_lookup(&sqlx_pool, &dl_row)
                .await
                .unwrap();
        }

        mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SEASON
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_THEME
        | mk_lib_common::mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER => {
            (metadata_uuid, guessit_data) = mk_lib_metadata::guessit::metadata_guessit(&sqlx_pool, &dl_row)
                .await
                .unwrap();
            if metadata_uuid == uuid::Uuid::nil() {
                metadata_uuid = mk_lib_metadata::tv::metadata_tv_lookup(&sqlx_pool, &dl_row, guessit_data)
                    .await
                    .unwrap();
            }
        }

        _ => eprintln!("que type does not equal any value"),
    }
    Ok(metadata_uuid)
}

/*

pub async fn metadata_identification(&sqlx_pool, dl_row, guessit_data):
    """
    Determine which provider to start lookup via class text
    """
    # else if class_text == "TV Extras":
    #     // include end slash so media doesn't get chopped up
    #     metadata_uuid = await &sqlx_pool.db_read_media_path_like(os.path.abspath(
    #         download_que_json["Path"].replace("/extras/", "/").rsplit("/", 1)[0]))
    #     if metadata_uuid != None:
    #         &sqlx_pool.db_download_delete(download_que_id)
    #     else:
    #         metadata_uuid = await metadata_tv.metadata_tv_lookup(&sqlx_pool,
    #                                                        dl_row,
    #                                                        guessit_data)

    # else if dl_row["mdq_que_type"] == common_global.DLMediaType.TV_Theme.value:
    #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident"})
    #     // include end slash so theme.mp3 doesn't get chopped up
    #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 2"})
    #     metadata_uuid = &sqlx_pool.db_read_media_path_like(os.path.abspath(
    #         download_que_json["Path"].replace(
    #             "/theme/", "/").replace("/backdrops/", "/")
    #             .rsplit("/", 1)[0]))
    #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 3"})
    #     if metadata_uuid != None:
    #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 4"})
    #         &sqlx_pool.db_download_delete(download_que_id)
    #     else:
    #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 5"})
    #         metadata_uuid = metadata_tv.metadata_tv_lookup(&sqlx_pool,
    #                                                        download_que_json,
    #                                                        download_que_id,
    #                                                        guessit_data)
    #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 6"})
    #         if metadata_uuid == None:
    #             await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 7"})
    #             // TODO so, the show hasn"t been fetched yet.....so, no path match
    #             &sqlx_pool.db_download_update_provider("ZZ", download_que_id)
    # else if dl_row["mdq_que_type"] == common_global.DLMediaType.TV_Trailer.value:
    #     // include end slash so theme.mp3 doesn't get chopped up
    #     metadata_uuid = &sqlx_pool.db_read_media_path_like(os.path.abspath(
    #         download_que_json["Path"].replace("/trailers/", "/").rsplit("/", 1)[0]))
    #     if metadata_uuid != None:
    #         &sqlx_pool.db_download_delete(download_que_id)
    #     else:
    #         metadata_uuid = metadata_tv.metadata_tv_lookup(&sqlx_pool,
    #                                                        dl_row,
    #                                                        guessit_data)
        }
    return metadata_uuid

 */
