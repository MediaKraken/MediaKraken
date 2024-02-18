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
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "ALTER TABLE mm_metadata_game_software_info \
            RENAME COLUMN gi_system_id TO gi_game_info_system_id;",
        )
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "INSERT INTO mm_metadata_game_systems_info \
            (gs_game_system_id, \
            gs_game_system_name, \
            gs_game_system_alias) \
            VALUES ($1, 'Arcade', 'Arcade')",
        )
        .bind(uuid::Uuid::nil())
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_hardware_manufacturer_name \
            ON mm_hardware_model USING btree (mm_hardware_manufacturer);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_hardware_model_type_name \
            ON mm_hardware_model USING btree (mm_hardware_model_type);",
        )
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "INSERT INTO mm_axum_users (anonymous, username, email, password) \
            values (true, 'Guest', 'guest@fake.com', crypt('fakepass', gen_salt('bf', 10)));",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_axum_user_permissions (\
            user_id INTEGER NOT NULL, \
            token VARCHAR(256) NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_axum_user_permissions_user_id \
            ON mm_axum_user_permissions USING btree (user_id);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 50).await?;
    }

    if version_no < 51 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_network_shares DROP COLUMN mm_network_share_xml;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_ip INET;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_path TEXT;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_comment TEXT;")
            .execute(&mut *transaction)
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
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX mm_backup_created_ndx \
           ON mm_backup USING btree (mm_backup_created);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 52).await?;
    }

    if version_no < 53 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("DROP TABLE IF EXISTS mm_user, mm_user_group, mm_user_profile;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 53).await?;
    }

    if version_no < 54 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("DROP INDEX IF EXISTS mm_metadata_game_systems_info_ndx_name;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS mm_metadata_game_systems_info_ndx_name ON mm_metadata_game_systems_info USING btree (gs_game_system_name);")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_metadata_game_systems_info ADD CONSTRAINT system_name_unique UNIQUE (gs_game_system_name);")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 54).await?;
    }

    if version_no < 55 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_library_dir ADD COLUMN mm_media_dir_share_guid uuid;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_media_dir_share_guid_ndx \
            ON mm_library_dir USING btree (mm_media_dir_share_guid);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 55).await?;
    }

    if version_no < 56 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_user text;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_password text;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_version smallint;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 56).await?;
    }

    if version_no < 57 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_network_shares DROP COLUMN mm_network_share_user;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares DROP COLUMN mm_network_share_password;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_user_guid uuid;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_share_auth (\
                mm_share_auth_guid uuid NOT NULL, \
                mm_share_auth_user TEXT NOT NULL, \
                mm_share_auth_password TEXT NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_share_auth_user_ndx \
                ON mm_share_auth USING btree (mm_share_auth_user);",
        )
        .execute(&mut *transaction)
        .await?;
        let new_guid = uuid::Uuid::new_v4();
        sqlx::query("INSERT INTO mm_share_auth (mm_share_auth_guid, mm_share_auth_user, mm_share_auth_password) \
            values ($1, 'guest', crypt('guest', gen_salt('bf', 10)));")
            .bind(new_guid)
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD COLUMN mm_network_share_workgroup text;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 57).await?;
    }

    if version_no < 58 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_network_shares ADD PRIMARY KEY (mm_network_share_guid);")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 58).await?;
    }

    if version_no < 59 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_library_dir DROP COLUMN mm_media_dir_username;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_library_dir DROP COLUMN mm_media_dir_password;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("ALTER TABLE mm_library_dir ADD COLUMN mm_media_dir_auth_user uuid;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_media_dir_auth_user_ndx \
            ON mm_library_dir USING btree (mm_media_dir_auth_user);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 59).await?;
    }

    if version_no < 60 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("ALTER TABLE mm_library_dir DROP COLUMN mm_media_dir_auth_user;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 60).await?;
    }

    if version_no < 61 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("CREATE EXTENSION IF NOT EXISTS citus WITH SCHEMA pg_catalog;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 61).await?;
    }

    if version_no < 62 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_openlib_author (\
                mm_openlib_author_id TEXT NOT NULL, \
                mm_openlib_author_json JSONB NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_author_id_ndx \
                ON mm_openlib_author USING btree (mm_openlib_author_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_author_json_ndx \
                ON mm_openlib_author USING gin (mm_openlib_author_json);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_openlib_edition (\
                mm_openlib_edition_id TEXT NOT NULL, \
                mm_openlib_edition_json JSONB NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_edition_id_ndx \
                ON mm_openlib_edition USING btree (mm_openlib_edition_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_edition_json_ndx \
            ON mm_openlib_edition USING gin (mm_openlib_edition_json);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_openlib_work (\
                mm_openlib_work_id TEXT NOT NULL, \
                mm_openlib_work_json JSONB NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_work_id_ndx \
                ON mm_openlib_work USING btree (mm_openlib_work_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_work_json_ndx \
            ON mm_openlib_work USING gin (mm_openlib_work_json);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 62).await?;
    }

    if version_no < 63 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mm_openlib_rating (\
                mm_openlib_rating_id TEXT NOT NULL, \
                mm_openlib_rating_json JSONB NOT NULL);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_rating_id_ndx \
                ON mm_openlib_rating USING btree (mm_openlib_rating_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS mm_openlib_rating_json_ndx \
                ON mm_openlib_rating USING gin (mm_openlib_rating_json);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 63).await?;
    }

    if version_no < 64 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("DROP INDEX IF EXISTS mm_openlib_author_id_ndx;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("DROP INDEX IF EXISTS mm_openlib_edition_id_ndx;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("DROP INDEX IF EXISTS mm_openlib_rating_id_ndx;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query("DROP INDEX IF EXISTS mm_openlib_work_id_ndx;")
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS mm_openlib_author_id_ndx \
                ON mm_openlib_author USING btree (mm_openlib_author_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS mm_openlib_edition_id_ndx \
                ON mm_openlib_edition USING btree (mm_openlib_edition_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS mm_openlib_rating_id_ndx \
                ON mm_openlib_rating USING btree (mm_openlib_rating_id);",
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            "CREATE UNIQUE INDEX IF NOT EXISTS mm_openlib_work_id_ndx \
                ON mm_openlib_work USING btree (mm_openlib_work_id);",
        )
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 64).await?;
    }

    if version_no < 65 {
        let mut transaction = sqlx_pool.begin().await?;
        sqlx::query("DROP TABLE IF EXISTS mm_openlib_rating;")
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        mk_lib_database_version_update(&sqlx_pool, 65).await?;
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
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
