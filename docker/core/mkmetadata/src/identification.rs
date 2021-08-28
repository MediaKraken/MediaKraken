use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[cfg(debug_assertions)]
#[path = "../../../../src/mk_lib_common/src/mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

#[cfg(not(debug_assertions))]
#[path = "mk_lib_common_enum_media_type.rs"]
mod mk_lib_common_enum_media_type;

pub async fn metadata_identification() {
    let mut metadata_uuid: Uuid;
    match dl_row["mdq_que_type"] {
        mk_lib_common_enum_media_type::DLMediaType::ADULT => {
            let metadata_uuid = metadata_adult::metadata_adult_lookup(db_connection,
                                                                      dl_row,
                                                                      guessit_file_name);
        }

        mk_lib_common_enum_media_type::DLMediaType::ANIME => {
            let metadata_uuid = metadata_anime::metadata_anime_lookup(db_connection,
                                                                      dl_row,
                                                                      guessit_file_name);
        }

        mk_lib_common_enum_media_type::DLMediaType::GAME_CHD => {
            let metadata_uuid = db_connection::db_meta_game_by_name_and_system(os.path.basename(
                os.path.splitext(dl_row["mdq_path"])[0]), lookup_system_id);
            if metadata_uuid == None {
                let sha1_value = common_hash.com_hash_sha1_c(dl_row["mdq_path"]);
                let metadata_uuid = db_connection::db_meta_game_by_sha1(sha1_value);
            }
        }

        mk_lib_common_enum_media_type::DLMediaType::GAME_ISO => {
            let metadata_uuid = db_connection::db_meta_game_by_name_and_system(os.path.basename(
                os.path.splitext(dl_row["mdq_path"])[0]), lookup_system_id);
            if metadata_uuid == None {
                let sha1_value = common_hash.com_hash_sha1_c(dl_row["mdq_path"]);
                let metadata_uuid = db_connection::db_meta_game_by_sha1(sha1_value);
            }
        }

        mk_lib_common_enum_media_type::DLMediaType::GAME_ROM => {
            let metadata_uuid = db_connection.db_meta_game_by_name_and_system(os.path.basename(
                os.path.splitext(dl_row["mdq_path"])[0]), lookup_system_id);
            if metadata_uuid == None {
                let sha1_hash = common_hash.com_hash_sha1_by_filename(dl_row["mdq_path"]);
                if sha1_hash != None {
                    let metadata_uuid = db_connection::db_meta_game_by_sha1(sha1_hash);
                }
            }
        }

        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION |
        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_BOOK |
        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_COMIC |
        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_COMIC_STRIP |
        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_MAGAZINE |
        mk_lib_common_enum_media_type::DLMediaType::PUBLICATION_GRAPHIC_NOVEL => {
            let metadata_uuid = metadata_periodicals::metadata_periodicals_lookup(db_connection,
                                                                                  dl_row);
        }

        mk_lib_common_enum_media_type::DLMediaType::MOVIE |
        mk_lib_common_enum_media_type::DLMediaType::MOVIE_EXTRAS |
        mk_lib_common_enum_media_type::DLMediaType::MOVIE_SUBTITLE |
        mk_lib_common_enum_media_type::DLMediaType::MOVIE_THEME |
        mk_lib_common_enum_media_type::DLMediaType::MOVIE_TRAILER => {
            let metadata_uuid = metadata_movie::metadata_movie_lookup(db_connection,
                                                                      dl_row,
                                                                      guessit_file_name);
        }

        mk_lib_common_enum_media_type::DLMediaType::MOVIE_HOME |
        mk_lib_common_enum_media_type::DLMediaType::PICTURE => {
            let metadata_uuid = uuid.uuid4();
        }

        mk_lib_common_enum_media_type::DLMediaType::MUSIC |
        mk_lib_common_enum_media_type::DLMediaType::MUSIC_ALBUM |
        mk_lib_common_enum_media_type::DLMediaType::MUSIC_SONG => {
            let metadata_uuid = metadata_music::metadata_music_lookup(db_connection,
                                                                      dl_row);
        }

        mk_lib_common_enum_media_type::DLMediaType::MUSIC_VIDEO => {
            let metadata_uuid = metadata_music_video.metadata_music_video_lookup(db_connection,
                                                                                 dl_row);
        }

        mk_lib_common_enum_media_type::DLMediaType::SPORTS => {
            let metadata_uuid = metadata_sports.metadata_sports_lookup(db_connection,
                                                                       dl_row);
        }

        mk_lib_common_enum_media_type::DLMediaType::TV |
        mk_lib_common_enum_media_type::DLMediaType::TV_EPISODE |
        mk_lib_common_enum_media_type::DLMediaType::TV_EXTRAS |
        mk_lib_common_enum_media_type::DLMediaType::TV_SEASON |
        mk_lib_common_enum_media_type::DLMediaType::TV_SUBTITLE |
        mk_lib_common_enum_media_type::DLMediaType::TV_THEME |
        mk_lib_common_enum_media_type::DLMediaType::TV_TRAILER => {
            let metadata_uuid = metadata_tv.metadata_tv_lookup(db_connection,
                                                               dl_row,
                                                               guessit_file_name);
        }

        _ => println!("que type does not equal any value"),
    }
}



    /*
        // if dl_row["mdq_que_type"] == common_global.DLMediaType.Music_Lyrics.value {
    //     // search musicbrainz as the lyrics should already be in the file / record
    //     pass;
    // }

    async def metadata_identification(db_connection, dl_row, guessit_file_name):
        """
        Determine which provider to start lookup via class text
        """
        # else if class_text == "TV Extras":
        #     # include end slash so media doesn"t get chopped up
        #     metadata_uuid = await db_connection.db_read_media_path_like(os.path.abspath(
        #         download_que_json["Path"].replace("/extras/", "/").rsplit("/", 1)[0]))
        #     if metadata_uuid is not None:
        #         db_connection.db_download_delete(download_que_id)
        #     else:
        #         metadata_uuid = await metadata_tv.metadata_tv_lookup(db_connection,
        #                                                        dl_row,
        #                                                        guessit_file_name)

        # else if dl_row["mdq_que_type"] == common_global.DLMediaType.TV_Theme.value:
        #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident"})
        #     # include end slash so theme.mp3 doesn"t get chopped up
        #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 2"})
        #     metadata_uuid = db_connection.db_read_media_path_like(os.path.abspath(
        #         download_que_json["Path"].replace(
        #             "/theme/", "/").replace("/backdrops/", "/")
        #             .rsplit("/", 1)[0]))
        #     await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 3"})
        #     if metadata_uuid is not None:
        #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 4"})
        #         db_connection.db_download_delete(download_que_id)
        #     else:
        #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 5"})
        #         metadata_uuid = metadata_tv.metadata_tv_lookup(db_connection,
        #                                                        download_que_json,
        #                                                        download_que_id,
        #                                                        guessit_file_name)
        #         await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 6"})
        #         if metadata_uuid is None:
        #             await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type="info", message_text= {"stuff": "tv theme ident 7"})
        #             # TODO so, the show hasn"t been fetched yet.....so, no path match
        #             db_connection.db_download_update_provider("ZZ", download_que_id)
        # else if dl_row["mdq_que_type"] == common_global.DLMediaType.TV_Trailer.value:
        #     # include end slash so theme.mp3 doesn"t get chopped up
        #     metadata_uuid = db_connection.db_read_media_path_like(os.path.abspath(
        #         download_que_json["Path"].replace("/trailers/", "/").rsplit("/", 1)[0]))
        #     if metadata_uuid is not None:
        #         db_connection.db_download_delete(download_que_id)
        #     else:
        #         metadata_uuid = metadata_tv.metadata_tv_lookup(db_connection,
        #                                                        dl_row,
        #                                                        guessit_file_name)
        else if dl_row["mdq_que_type"] == common_global.DLMediaType.Game.value {
            metadata_uuid = await metadata_game.metadata_game_lookup(db_connection,
                                                                     dl_row);
                                                                     }
        else if dl_row["mdq_que_type"] == common_global.DLMediaType.Game_Intro.value {
            pass;
            }
        else if dl_row["mdq_que_type"] == common_global.DLMediaType.Game_Speedrun.value {
            pass;
            }
        else if dl_row["mdq_que_type"] == common_global.DLMediaType.Game_Superplay.value {
            pass;
            }
        return metadata_uuid

     */