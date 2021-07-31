use tokio_postgres::Error;
use uuid::Uuid;

pub async fn mk_lib_database_hardware_manufacturer_upsert(client: &tokio_postgres::Client,
                                                          manufacturer_name: String,
                                                          manufacturer_id: i16)
                                                          -> Result<(), Error> {
    client
        .query("INSERT INTO mm_hardware_manufacturer (mm_hardware_manu_guid,\
         mm_hardware_manu_name, mm_hardware_manu_gc_id) VALUES($1, $2, $3)\
          ON CONFLICT (mm_hardware_manu_name) DO NOTHING;",
               &[&Uuid::new_v4(), &manufacturer_name, &manufacturer_id]).await?;
    Ok(())
}