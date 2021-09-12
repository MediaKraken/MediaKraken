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