use sqlx::postgres::PgRow;
use uuid::Uuid;

pub async fn mk_lib_database_hardware_manufacturer_upsert(pool: &sqlx::PgPool,
                                                          manufacturer_name: String,
                                                          manufacturer_id: i32)
                                                          -> Result<(), sqlx::Error> {
    println!("here {:?} {:?}", manufacturer_name, manufacturer_id);
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_hardware_manufacturer (mm_hardware_manu_guid, \
        mm_hardware_manu_name, mm_hardware_manu_gc_id) values ($1, $2, $3) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(Uuid::new_v4())
        .bind(manufacturer_name)
        .bind(manufacturer_id)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_type_upsert(pool: &sqlx::PgPool,
                                                  hardware_type: String)
                                                  -> Result<(), sqlx::Error> {
    println!("here2 {:?}", hardware_type);
    let mut transaction = pool.begin().await?;
    sqlx::query("insert into mm_hardware_type (mm_hardware_type_guid, \
        mm_hardware_type_name) values ($1, $2) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(Uuid::new_v4())
        .bind(hardware_type)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}

/*
async def db_hardware_device_count(self, hardware_manufacturer, model_name=None,
                                   db_connection=None):
    """
    Return json for machine/model
    """
    if model_name is None:
        return await db_conn.fetchval('select count(*) from mm_hardware'
                                      ' where mm_hardware_manufacturer = $1',
                                      hardware_manufacturer)
    else:
        return await db_conn.fetchval('select count(*) from mm_hardware'
                                      ' where mm_hardware_manufacturer = $1'
                                      ' and mm_hardware_model = $2',
                                      hardware_manufacturer, model_name)


async def db_hardware_json_read(self, manufacturer, model_name, db_connection=None):
    """
    Return json for machine/model
    """
    return await db_conn.fetchval('select mm_hardware_json'
                                  ' from mm_hardware_json'
                                  ' where mm_hardware_manufacturer = $1'
                                  ' and mm_hardware_model = $2',
                                  manufacturer, model_name)


async def db_hardware_insert(self, manufacturer, model_name, json_data, db_connection=None):
    new_guid = uuid.uuid4()
    await db_conn.execute('insert into mm_hardware_json (mm_hardware_id,'
                          ' mm_hardware_manufacturer,'
                          ' mm_hardware_model,'
                          ' mm_hardware_json)'
                          ' values ($1, $2, $3, $4)',
                          new_guid, manufacturer, model_name, json_data)
    await db_conn.execute('commit')
    return new_guid


async def db_hardware_delete(self, guid, db_connection=None):
    await db_conn.execute('delete from mm_hardware_json'
                          ' where mm_hardware_id = $1', guid)
    await db_conn.execute('commit')

 */