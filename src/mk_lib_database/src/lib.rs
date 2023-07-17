pub mod mk_lib_database;
pub mod mk_lib_database_backup;
pub mod mk_lib_database_cron;
pub mod mk_lib_database_game_servers;
pub mod mk_lib_database_hardware_device;
pub mod mk_lib_database_library;
pub mod mk_lib_database_link_server;
pub mod mk_lib_database_network_share;
pub mod mk_lib_database_notification;
pub mod mk_lib_database_option_status;
pub mod mk_lib_database_postgresql;
pub mod mk_lib_database_search;
pub mod mk_lib_database_sync;
pub mod mk_lib_database_usage;
pub mod mk_lib_database_user_activity;
pub mod mk_lib_database_user;
pub mod mk_lib_database_user_profile;
pub mod mk_lib_database_user_queue;
pub mod mk_lib_database_version;
pub mod mk_lib_database_version_schema;

#[path = "media"]
pub mod database_media {
    pub mod mk_lib_database_media;
    pub mod mk_lib_database_media_adult;
    pub mod mk_lib_database_media_anime;
    pub mod mk_lib_database_media_book;
    pub mod mk_lib_database_media_game;
    pub mod mk_lib_database_media_game_system;
    pub mod mk_lib_database_media_home_media;
    pub mod mk_lib_database_media_images;
    pub mod mk_lib_database_media_iradio;
    pub mod mk_lib_database_media_movie;
    pub mod mk_lib_database_media_music;
    pub mod mk_lib_database_media_music_video;
    pub mod mk_lib_database_media_remote;
    pub mod mk_lib_database_media_sports;
    pub mod mk_lib_database_media_tv;
}

#[path = "metadata"]
pub mod database_metadata {
    pub mod mk_lib_database_metadata;
    pub mod mk_lib_database_metadata_adult;
    pub mod mk_lib_database_metadata_anime;
    pub mod mk_lib_database_metadata_book;
    pub mod mk_lib_database_metadata_collection;
    pub mod mk_lib_database_metadata_download_queue;
    pub mod mk_lib_database_metadata_game;
    pub mod mk_lib_database_metadata_game_system;
    pub mod mk_lib_database_metadata_image;
    pub mod mk_lib_database_metadata_movie;
    pub mod mk_lib_database_metadata_music;
    pub mod mk_lib_database_metadata_music_brainz;
    pub mod mk_lib_database_metadata_music_video;
    pub mod mk_lib_database_metadata_person;
    pub mod mk_lib_database_metadata_review;
    pub mod mk_lib_database_metadata_sports;
    pub mod mk_lib_database_metadata_tv;
    pub mod mk_lib_database_metadata_tv_live;
}
