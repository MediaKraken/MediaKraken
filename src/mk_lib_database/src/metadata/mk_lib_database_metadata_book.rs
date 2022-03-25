use uuid::Uuid;
use sqlx::postgres::PgRow;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_metadata_book_by_uuid(pool: &sqlx::PgPool, book_uuid: uuid::Uuid)
                                                   -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mm_metadata_book_json from mm_metadata_book \
        where mm_metadata_book_guid = $1")
        .bind(book_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaBookList {
	mm_metadata_book_guid: uuid::Uuid,
	mm_metadata_book_name: String,
}

pub async fn mk_lib_database_metadata_book_read(pool: &sqlx::PgPool,
                                                search_value: String,
                                                offset: i32, limit: i32)
                                                -> Result<Vec<DBMetaBookList>, sqlx::Error> {
    // TODO sort by release date
    let mut select_query;
    if search_value != "" {
        select_query = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book where mm_metadata_book_name % $1 \
            order by mm_metadata_book_name offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit);
    } else {
        select_query = sqlx::query("select mm_metadata_book_guid, mm_metadata_book_name \
            from mm_metadata_book order by mm_metadata_book_name \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit);
    }
    let table_rows: Vec<DBMetaBookList> = select_query
        .map(|row: PgRow| DBMetaBookList {
            mm_metadata_book_guid: row.get("mm_metadata_book_guid"),
            mm_metadata_book_name: row.get("mm_metadata_book_name"),
        })
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_book_count(pool: &sqlx::PgPool,
                                                 search_value: String)
                                                 -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book \
            where mm_metadata_book_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_book")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

/*

def db_meta_book_guid_by_isbn(self, isbn_uuid, isbn13_uuid):
    """
    # metadata guid by isbm id
    """
    self.db_cursor.execute('select mm_metadata_book_guid'
                           ' from mm_metadata_book'
                           ' where mm_metadata_book_isbn = %s or mm_metadata_book_isbn13 = %s',
                           (isbn_uuid, isbn13_uuid))
    try:
        return self.db_cursor.fetchone()['mm_metadata_book_guid']
    except:
        return None


def db_meta_book_guid_by_name(self, book_name):
    """
    # metadata guid by name
    """
    # TODO can be more than one by name
    # TODO sort by release date
    self.db_cursor.execute('select mm_metadata_book_guid'
                           ' from mm_metadata_book'
                           ' where mm_metadata_book_name = %s', (book_name,))
    try:
        return self.db_cursor.fetchone()['mm_metadata_book_guid']
    except:
        return None


def db_meta_book_insert(self, json_data):
    """
    # insert metadata json from isbndb
    """
    # json_data = json.dumps(json_data)
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info',
                                                         message_text={'book insert': json_data})
    common_logging_elasticsearch_httpx.com_es_httpx_post(message_type='info', message_text={
        'book insert data': json_data['data']})
    insert_uuid = uuid.uuid4()
    self.db_cursor.execute('insert into mm_metadata_book (mm_metadata_book_guid,'
                           ' mm_metadata_book_isbn,'
                           ' mm_metadata_book_isbn13,'
                           ' mm_metadata_book_name,'
                           ' mm_metadata_book_json)'
                           ' values (%s,%s,%s,%s,%s)',
                           (insert_uuid, json_data['data'][0]['isbn10'],
                            json_data['data'][0]['isbn13'], json_data['data'][0]['title'],
                            json.dumps(json_data['data'][0])))
    self.db_commit()
    return insert_uuid



def db_meta_book_image_random(self, return_image_type='Cover'):
    """
    Find random book image
    """
    # TODO little bobby tables
    self.db_cursor.execute(
        'select mm_metadata_book_localimage_json->\'Images\'->\'themoviedb\'->>\''
        + return_image_type + '\' as image_json,mm_metadata_book_guid'
                              ' from mm_media,mm_metadata_book'
                              ' where mm_media_metadata_guid = mm_metadata_book_guid'
                              ' and (mm_metadata_book_localimage_json->\'Images\'->\'themoviedb\'->>\''
        + return_image_type + '\'' + ')::text != \'null\''
                                     ' order by random() limit 1')
    try:
        # then if no results.....a None will except which will then pass None, None
        image_json, metadata_id = self.db_cursor.fetchone()
        return image_json, metadata_id
    except:
        return None, None

 */