use uuid::Uuid;

pub async fn mk_lib_database_hardware_manufacturer_upsert(pool: &sqlx::PgPool,
                                                          manufacturer_name: String,
                                                          manufacturer_id: i32)
                                                          -> Result<(), sqlx::Error> {
    println!("here {:?} {:?}", manufacturer_name, manufacturer_id);
    sqlx::query("insert into mm_hardware_manufacturer (mm_hardware_manu_guid, \
        mm_hardware_manu_name, mm_hardware_manu_gc_id) values ($1, $2, $3) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(Uuid::new_v4(), manufacturer_name, manufacturer_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_type_upsert(pool: &sqlx::PgPool,
                                                  hardware_type: String)
                                                  -> Result<(), sqlx::Error> {
    println!("here2 {:?}", hardware_type);
    sqlx::query("insert into mm_hardware_type (mm_hardware_type_guid, \
        mm_hardware_type_name) values ($1, $2) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING")
        .bind(Uuid::new_v4(), hardware_type)
        .execute(pool)
        .await?;
    Ok(())
}