use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;
use rocket_dyn_templates::serde::{Serialize, Deserialize};

pub async fn mk_lib_database_metadata_exists_person(pool: &sqlx::PgPool,
                                                    metadata_id: i32)
                                                    -> Result<bool, sqlx::Error> {
    let row: (bool, ) = sqlx::query_as("select exists(select 1 from mm_metadata_person \
        where mm_metadata_person_media_id = $1 limit 1) as found_record limit 1")
        .bind(metadata_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn mk_lib_database_metadata_person_count(pool: &sqlx::PgPool,
                                                   search_value: String)
                                                   -> Result<(i32), sqlx::Error> {
    if search_value != "" {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_person \
            where mmp_person_name % $1")
            .bind(search_value)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    } else {
        let row: (i32, ) = sqlx::query("select count(*) from mm_metadata_person")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaPersonList {
	mmp_id: uuid::Uuid,
	mmp_person_name: String,
	mmp_person_image: String,
	mmp_profile: String,
}

pub async fn mk_lib_database_metadata_person_read(pool: &sqlx::PgPool,
                                                  search_value: String,
                                                  offset: i32, limit: i32)
                                                  -> Result<Vec<PgRow>, sqlx::Error> {
    // TODO order by birth date
    if search_value != "" {
        let rows = sqlx::query("select mmp_id, mmp_person_name, mmp_person_image, \
            mmp_person_meta_json->\'profile_path\' as mmp_profile \
            from mm_metadata_person where mmp_person_name % $1 \
            order by LOWER(mmp_person_name) offset $2 limit $3")
            .bind(search_value)
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    } else {
        let rows = sqlx::query("select mmp_id,mmp_person_name, mmp_person_image, \
            mmp_person_meta_json->\'profile_path\' as mmp_meta \
            from mm_metadata_person order by LOWER(mmp_person_name) \
            offset $1 limit $2")
            .bind(offset)
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

pub async fn mk_lib_database_meta_person_by_guid(pool: &sqlx::PgPool,
                                                 person_uuid: uuid::Uuid)
                                                 -> Result<Vec<PgRow>, sqlx::Error> {
    let rows: Vec<PgRow> = sqlx::query("select mmp_id, mmp_person_media_id, \
        mmp_person_meta_json, mmp_person_image, mmp_person_name, \
        mmp_person_meta_json->'profile_path' as mmp_meta \
        from mm_metadata_person where mmp_id = $1")
        .bind(person_uuid)
        .fetch_one(pool)
        .await?;
    Ok(rows)
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DBMetaPersonNameList {
	mmp_id: uuid::Uuid,
	mmp_person_media_id: String,
	mmp_person_meta_json: String,
	mmp_person_image: String,
    mmp_person_name: String,
}

pub async fn mk_lib_database_meta_person_by_name(pool: &sqlx::PgPool,
                                                 person_name: String)
                                                 -> Result<Vec<DBMetaPersonNameList>, sqlx::Error> {
    let select_query = sqlx::query("select mmp_id, mmp_person_media_id, \
        mmp_person_meta_json, \
        mmp_person_image, \
        mmp_person_name \
        from mm_metadata_person \
        where mmp_person_name = %s")
        .bind(person_name);
    let table_rows: Vec<DBMetaPersonNameList> = select_query
		.map(|row: PgRow| DBMetaPersonNameList {
			mmp_id: row.get("mmp_id"),
			mmp_person_media_id: row.get("mmp_person_media_id"),
			mmp_person_meta_json: row.get("mmp_person_meta_json"),
			mmp_person_image: row.get("mmp_person_image"),
			mmp_person_name: row.get("mmp_person_name"),
		})
        .fetch_all(pool)
        .await?;
    Ok(table_rows)
}

pub async fn mk_lib_database_metadata_person_insert(pool: &sqlx::PgPool,
                                                    person_name:String,
                                                    media_id:String,
                                                    person_json: Json,
                                                    person_image_path:String)
                                                    -> Result<(uuid::Uuid), sqlx::Error> {
    let new_guid = Uuid::new_v4();
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_metadata_person (mmp_id, mmp_person_name, \
        mmp_person_media_id, mmp_person_meta_json, \
        mmp_person_image) \
        values ($1,$2,$3,$4,$5)")
        .bind(new_guid)
        .bind(person_name)
        .bind(media_id)
        .bind(person_json)
        .bind(person_image_path)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(new_guid)
}

/*

// TODO port query
async def db_meta_person_as_seen_in(self, person_guid, db_connection=None):
    """
    # find other media for person
    """
    row_data = await self.db_meta_person_by_guid(guid=person_guid, db_connection=db_conn)
    if row_data is None:  # exit on not found person
        return None
    # TODO jin index the credits
    return await db_conn.fetch('select mm_metadata_guid,mm_metadata_name,'
                               ' mm_metadata_localimage_json->\'Poster\''
                               ' from mm_metadata_movie'
                               ' where mm_metadata_json->\'credits\'->\'cast\''
                               ' @> \'[{"id": '
                               + str(row_data['mmp_person_media_id'])
                               + '}]\' order by LOWER(mm_metadata_name)')


// TODO port query
async def db_meta_person_update(self, provider_name, provider_uuid, person_bio, person_image,
                                db_connection=None):
    """
    update the person bio/etc
    """
    await db_conn.execute('update mm_metadata_person set mmp_person_meta_json = $1,'
                          ' mmp_person_image = $2'
                          ' where mmp_person_media_id = $3',
                          person_bio, person_image, provider_uuid)
    await db_conn.execute('commit')


// TODO port query
async def db_meta_person_insert_cast_crew(self, meta_type, person_json, db_connection=None):
    """
    # batch insert from json of crew/cast
    """
    # TODO failing due to only one person in json?  hence pulling id, etc as the for loop
    multiple_person = False
    try:
        for person_data in person_json:
            multiple_person = True
    except:
        pass
    if multiple_person:
        for person_data in person_json:
            await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                             message_text={
                                                                                 "person data": person_data})
            if meta_type == "themoviedb":
                person_id = person_data['id']
                person_name = person_data['name']
            else:
                person_id = None
                person_name = None
            if person_id is not None:
                # TODO do an upsert instead
                if await self.db_meta_person_id_count(person_id) is True:
                    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                        message_type='info',
                        message_text={
                            'db_meta_person_insert_cast_crew': "skip insert as person exists"})
                else:
                    new_guid = uuid.uuid4()
                    # Shouldn't need to verify fetch doesn't exist as the person insert
                    # is right below.  As then the next person record read will find
                    # the inserted record.
                    # insert download record for bio/info
                    await self.db_download_insert(provider=meta_type,
                                                  que_type=common_global.DLMediaType.Person.value,
                                                  down_json={"Status": "Fetch",
                                                             "ProviderMetaID": person_id},
                                                  down_new_uuid=new_guid,
                                                  db_connection=db_connection)
                    # insert person record
                    await self.db_meta_person_insert(uuid_id=new_guid,
                                                     person_name=person_name,
                                                     media_id=person_id,
                                                     person_json=None,
                                                     image_path=None,
                                                     db_connection=db_connection)
    else:
        if meta_type == "themoviedb":
            # cast/crew can exist but be blank
            try:
                person_id = person_json['id']
                person_name = person_json['name']
            except:
                person_id = None
                person_name = None
        else:
            person_id = None
            person_name = None
        if person_id is not None:
            # TODO upsert instead
            if await self.db_meta_person_id_count(person_id) is True:
                await common_logging_elasticsearch_httpx.com_es_httpx_post_async(
                    message_type='info',
                    message_text={'stuff': "skippy"})
            else:
                new_guid = uuid.uuid4()
                # Shouldn't need to verify fetch doesn't exist as the person insert
                # is right below.  As then the next person record read will find
                # the inserted record.
                # insert download record for bio/info
                await self.db_download_insert(provider=meta_type,
                                              que_type=common_global.DLMediaType.Person.value,
                                              down_json={"Status": "Fetch",
                                                         "ProviderMetaID": person_id},
                                              down_new_uuid=new_guid,
                                              db_connection=db_connection)
                # insert person record
                await self.db_meta_person_insert(uuid_id=new_guid,
                                                 person_name=person_name,
                                                 media_id=person_id,
                                                 person_json=None,
                                                 image_path=None,
                                                 db_connection=db_connection)

 */