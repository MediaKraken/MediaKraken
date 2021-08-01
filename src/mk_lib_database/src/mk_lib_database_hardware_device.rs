use tokio_postgres::Error;
use uuid::Uuid;

pub async fn mk_lib_database_hardware_manufacturer_upsert(client: &tokio_postgres::Client,
                                                          manufacturer_name: String,
                                                          manufacturer_id: i32)
                                                          -> Result<(), Error> {
    println!("here {:?} {:?}", manufacturer_name, manufacturer_id);
    client
        .query("insert into mm_hardware_manufacturer (mm_hardware_manu_guid, \
        mm_hardware_manu_name, mm_hardware_manu_gc_id) values ($1, $2, $3) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING",
               &[&Uuid::new_v4(), &manufacturer_name, &manufacturer_id]).await?;
    Ok(())
}

pub async fn mk_lib_database_hardware_type_upsert(client: &tokio_postgres::Client,
                                                  hardware_type: String)
                                                  -> Result<(), Error> {
    println!("here2 {:?}", hardware_type);
    client
        .query("insert into mm_hardware_type (mm_hardware_type_guid, \
        mm_hardware_type_name) values ($1, $2) \
        ON CONFLICT (mm_hardware_manu_name) DO NOTHING",
               &[&Uuid::new_v4(), &hardware_type]).await?;
    Ok(())
}