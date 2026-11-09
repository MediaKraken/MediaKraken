use sqlx::postgres::PgRow;

pub async fn mk_lib_database_remote_media_count(
    sqlx_pool: &sqlx::PgPool,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(r#"select count(*) from mm_media_remote"#)
        .fetch_one(sqlx_pool)
        .await?;
    Ok(row.0)
}

/// Insert a remote media record; returns the new guid.
///
/// NOTE: the Python original used `mmr_media_link_id` for the link column;
/// ported as `mmr_media_link_guid`. `mmr_media_class_guid` follows the live
/// idiom (`mk_lib_database_media_movie_genre_count`,
/// `mk_lib_database_media_home_media_count`), which bind the `DLMediaType`
/// i16 to it; the schema dump lists `mmr_media_class_enum` instead (confirm
/// against the live database).
pub async fn mk_lib_database_media_remote_insert(
    sqlx_pool: &sqlx::PgPool,
    media_link_guid: uuid::Uuid,
    media_guid: uuid::Uuid,
    media_class: i16,
    media_metadata_guid: uuid::Uuid,
    media_ffprobe_json: &serde_json::Value,
) -> Result<uuid::Uuid, sqlx::Error> {
    let new_guid = uuid::Uuid::now_v7();
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query(
        r#"insert into mm_media_remote (mmr_media_guid, mmr_media_link_guid,
        mmr_media_uuid, mmr_media_class_guid, mmr_media_metadata_guid,
        mmr_media_ffprobe_json)
        values ($1,$2,$3,$4,$5,$6)"#,
    )
    .bind(new_guid)
    .bind(media_link_guid)
    .bind(media_guid)
    .bind(media_class)
    .bind(media_metadata_guid)
    .bind(media_ffprobe_json)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/// Remote media records, all columns.
///
/// With a guid, returns at most one row; without, returns all rows.
pub async fn mk_lib_database_media_remote_read(
    sqlx_pool: &sqlx::PgPool,
    media_guid: Option<uuid::Uuid>,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = match media_guid {
        Some(guid) => {
            sqlx::query(r#"select * from mm_media_remote where mmr_media_guid = $1"#)
                .bind(guid)
                .fetch_all(sqlx_pool)
                .await?
        }
        None => {
            sqlx::query(r#"select * from mm_media_remote"#)
                .fetch_all(sqlx_pool)
                .await?
        }
    };
    Ok(rows)
}

/*
// Not ported: fully commented out in the Python original (dead in the dead);
// processed via main_link and depends on per-class metadata guid lookups.

# processed via main_link........
# process new records from network sync event from linked server
# def db_Media_Remote_New_Data(self, link_uuid, link_records):
#    # 0-media guid, 1-type, 2-ffrobe, 3-media id json
#    metadata_guid = None
#    for row_data in link_records:
#        if row_data[1] == 'Movie':
#            if 'themoviedb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_tmdb(row_data[3]['themoviedb'])
#            if metadata_guid is None and 'imdb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_imdb(row_data[3]['imdb'])
#            if metadata_guid is None and 'thetvdb' in row_data[3]:
#                metadata_guid = db_meta_guid_by_tvdb(row_data[3]['thetvdb'])
#        else if row_data[1] == 'TV Show':
#            if 'imdb' in row_data[3]
#                metadata_guid = db_metaTV_guid_by_imdb(row_data[3]['imdb'])
#            if metadata_guid is None and 'thetvdb' in row_data[3]:
#                metadata_guid = db_metatv_guid_by_tvdb(row_data[3]['thetvdb'])
#            if metadata_guid is None and 'tvmaze' in row_data[3]:
#                metadata_guid = db_metaTV_guid_by_tvmaze(row_data[3]['tvmaze'])
#        else if row_data[1] == 'Sports':
#            pass
#        else if row_data[1] == 'Music':
#            pass
#        else if row_data[1] == 'Music Video':
#            pass
#        else if row_data[1] == 'Book':
#            pass
#        else:
#            common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='error', message_text={'stuff':'Link bad data type: %s', row_data[1])
#            return None
#        if metadata_guid != None:
#            self.db_insert_remote_media(link_uuid, row_data[0], \
# self.db_media_uuid_by_class(row_data[1]), metadata_guid[0], json.dumps(row_data[2]))
*/

/// New media for a link since the last sync, across the enabled media classes.
///
/// NOTE: the Python self-joins were tautologies
/// (`mm_metadata_tvshow_guid = mm_metadata_tvshow_guid` etc.); all branches
/// join on `mm_media_metadata_guid = mm_metadata_<class>_guid`. The fourth
/// union column (int/jsonb/text mix) is cast to text in every branch.
/// TODO add games to this
#[allow(clippy::too_many_arguments)]
pub async fn mk_lib_database_media_remote_read_new(
    sqlx_pool: &sqlx::PgPool,
    date_last_sync: &str,
    sync_movie: bool,
    sync_tv: bool,
    sync_sports: bool,
    sync_music: bool,
    sync_music_video: bool,
    sync_book: bool,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new(String::new());
    let mut first = true;
    let push_branch = |query: &mut sqlx::QueryBuilder<sqlx::Postgres>,
                       first: &mut bool,
                       class_name: &str,
                       table: &str,
                       metadata_guid_col: &str,
                       media_id_col: &str| {
        if !*first {
            query.push(" union all ");
        }
        query.push("select mm_media_guid, ");
        query.push_bind(class_name);
        query.push(", mm_media_ffprobe_json, ");
        query.push(media_id_col);
        query.push("::text from mm_media, ");
        query.push(table);
        query.push(" where mm_media_metadata_guid = ");
        query.push(metadata_guid_col);
        query.push(" and mm_media_json->>'DateAdded' >= ");
        query.push_bind(date_last_sync);
        *first = false;
    };
    if sync_movie {
        push_branch(
            &mut query,
            &mut first,
            "Movie",
            "mm_metadata_movie",
            "mm_metadata_movie_guid",
            "mm_metadata_movie_media_id",
        );
    }
    if sync_tv {
        push_branch(
            &mut query,
            &mut first,
            "TV Show",
            "mm_metadata_tvshow",
            "mm_metadata_tvshow_guid",
            "mm_metadata_media_tvshow_id",
        );
    }
    if sync_sports {
        push_branch(
            &mut query,
            &mut first,
            "Sports",
            "mm_metadata_sports",
            "mm_metadata_sports_guid",
            "mm_metadata_media_sports_id",
        );
    }
    if sync_music {
        push_branch(
            &mut query,
            &mut first,
            "Music",
            "mm_metadata_music",
            "mm_metadata_music_guid",
            "mm_metadata_media_music_id",
        );
    }
    if sync_music_video {
        push_branch(
            &mut query,
            &mut first,
            "Music Video",
            "mm_metadata_music_video",
            "mm_metadata_music_video_guid",
            "mm_metadata_music_video_media_id",
        );
    }
    if sync_book {
        push_branch(
            &mut query,
            &mut first,
            "Book",
            "mm_metadata_book",
            "mm_metadata_book_guid",
            "mm_metadata_book_isbn",
        );
    }
    if first {
        return Ok(Vec::new());
    }
    query.build().fetch_all(sqlx_pool).await
}
