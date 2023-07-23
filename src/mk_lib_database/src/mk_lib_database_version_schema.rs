use crate::mk_lib_database_option_status;
use serde_json::Value;

pub async fn mk_lib_database_update_schema(
    sqlx_pool: &sqlx::PgPool,
    version_no: i32,
) -> Result<bool, sqlx::Error> {
    if version_no < 44 {
        // set mame version to 240
        let _option_json: Value =
            mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool)
                .await
                .unwrap();
        // option_json["MAME"]["Version"] = 240;
        // mk_lib_database_option_status::mk_lib_database_option_update(&sqlx_pool, option_json).await?;
        mk_lib_database_version_update(&sqlx_pool, 44).await?;
    }
    if version_no < 45 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_software_info \
            RENAME COLUMN gi_id TO gi_game_info_id;",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_software_info \
            RENAME COLUMN gi_system_id TO gi_game_info_system_id;",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 45).await?;
    }
    if version_no < 46 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_systems_info \
            RENAME COLUMN gs_id TO gs_game_system_id;",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "INSERT INTO mm_metadata_game_systems_info \
            (gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias) \
            VALUES ($1, 'Arcade', 'Arcade')",
        )
        .bind(uuid::Uuid::nil())
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 46).await?;
    }
    if version_no < 47 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE mm_hardware_model (
                mm_hardware_model_guid uuid NOT NULL,
                mm_hardware_manufacturer text,
                mm_hardware_model_type text,
                mm_hardware_model_name text
            );",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 47).await?;
    }
    if version_no < 48 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "ALTER TABLE ONLY mm_hardware_model
            ADD CONSTRAINT mm_hardware_model_guid_pk PRIMARY KEY (mm_hardware_model_guid);",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_hardware_manufacturer_name \
            ON mm_hardware_model USING btree (mm_hardware_manufacturer);",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_hardware_model_type_name \
            ON mm_hardware_model USING btree (mm_hardware_model_type);",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 48).await?;
    }
    if version_no < 49 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_network_shares (\
            mm_network_share_guid uuid NOT NULL, \
            mm_network_share_xml text);",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 49).await?;
    }

    if version_no < 50 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_axum_users (\
            id bigint Primary Key Generated Always as Identity, \
            anonymous BOOLEAN NOT NULL, \
            username VARCHAR(256) NOT NULL, \
            password TEXT NOT NULL, \
            email VARCHAR(256), \
            last_signin timestamp, \
            last_signoff timestamp);",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "INSERT INTO mm_axum_users (anonymous, username, email, password) \
            values (true, 'Guest', 'guest@fake.com', crypt('fakepass', gen_salt('bf', 10)));",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_axum_user_permissions (\
            user_id INTEGER NOT NULL, \
            token VARCHAR(256) NOT NULL);",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_axum_user_permissions_user_id \
            ON mm_axum_user_permissions USING btree (user_id);",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 50).await?;
    }

    if version_no < 51 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_network_shares DROP COLUMN mm_network_share_xml;")
            .execute(&mut transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_ip INET;")
            .execute(&mut transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_path TEXT;")
            .execute(&mut transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_comment TEXT;")
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 51).await?;
    }

    if version_no < 52 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_backup (\
                mm_backup_guid uuid NOT NULL, \
                mm_backup_description TEXT NOT NULL, \
                mm_backup_location_type integer NOT NULL, \
                mm_backup_location TEXT NOT NULL, \
                mm_backup_created timestamp NOT NULL);",
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_backup_created_ndx \
           ON mm_backup USING btree (mm_backup_created);",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 52).await?;
    }

    if version_no < 53{
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "DROP TABLE IF EXISTS mm_user, mm_user_group, mm_user_profile;",
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 53).await?;
    }

    Ok(true)
}

pub async fn mk_lib_database_version_update(
    sqlx_pool: &sqlx::PgPool,
    version_number: i32,
) -> Result<(), sqlx::Error> {
    let mut transaction = sqlx_pool.begin().await?;
    sqlx::query("update mm_version set mm_version_number = $1")
        .bind(version_number)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
