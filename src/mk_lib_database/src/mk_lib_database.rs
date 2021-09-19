use std::env;
use std::fs;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;

pub async fn mk_lib_database_open_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    // trim is get rid of the \r returned in hostname
    let hostname: String = sys_info::hostname().unwrap().trim().to_string();
    println!("Host: {}", hostname);
    let mut connection_string: String;
    if hostname == "wsripper2" {
         connection_string = "postgresql://postgres:metaman@localhost/postgres".to_string();
    } else if hostname == "th-hplaptop-1" || hostname == "th-hplap-1"
        || hostname == "th-linuxgui-1" {
         connection_string = "postgresql://postgres:metaman@th-docker-1/postgres".to_string();
    } else if Path::new("/run/secrets/db_password").exists() {
        let dp_pass = fs::read_to_string("/run/secrets/db_password").unwrap();
         connection_string = format!("postgresql://postgres:{}@mkstack_database/postgres",
                                    dp_pass);
    } else {
        let dp_pass = env::var("POSTGRES_PASSWORD").unwrap();
         connection_string = format!("postgresql://postgres:{}@mkstack_database/postgres",
                                    dp_pass);
    }
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&connection_string).await?;
    Ok(pool)
}

/*
async def db_table_index_check(self, resource_name, db_connection=None):
    """
    # check for table or index
    """
    # TODO little bobby tables
    await self.db_cursor.execute('SELECT to_regclass(\'public.$1\')', resource_name)
    return await self.db_cursor.fetchval()

 */