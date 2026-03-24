use serde_json::{Value, json};
use sqlx::{FromRow, postgres::PgPool};

pub const DEFAULT_PAGINATION_COUNT: i64 = 30;
pub const MIN_PAGINATION_COUNT: i64 = 5;
pub const MAX_PAGINATION_COUNT: i64 = 200;

#[derive(FromRow)]
struct UserProfileRow {
    mm_user_profile_guid: uuid::Uuid,
    mm_user_profile_json: Option<Value>,
}

pub fn normalize_pagination_count(value: i64) -> i64 {
    value.clamp(MIN_PAGINATION_COUNT, MAX_PAGINATION_COUNT)
}

pub async fn load_user_pagination_count(
    sqlx_pool: &PgPool,
    user_id: i64,
) -> Result<i64, sqlx::Error> {
    let profile_name = profile_name_for_user(user_id);
    let row = sqlx::query_scalar::<_, Option<String>>(
        r#"
        select mm_user_profile_json ->> 'pagination_count'
        from mm_user_profile
        where mm_user_profile_name = $1
        order by mm_user_profile_guid
        limit 1
        "#,
    )
    .bind(profile_name)
    .fetch_optional(sqlx_pool)
    .await?;

    let pagination_count = row
        .flatten()
        .and_then(|value| value.parse::<i64>().ok())
        .map(normalize_pagination_count)
        .unwrap_or(DEFAULT_PAGINATION_COUNT);

    Ok(pagination_count)
}

pub async fn upsert_user_pagination_count(
    sqlx_pool: &PgPool,
    user_id: i64,
    pagination_count: i64,
) -> Result<(), sqlx::Error> {
    let profile_name = profile_name_for_user(user_id);
    let existing_profile = sqlx::query_as::<_, UserProfileRow>(
        r#"
        select mm_user_profile_guid, mm_user_profile_json
        from mm_user_profile
        where mm_user_profile_name = $1
        order by mm_user_profile_guid
        limit 1
        "#,
    )
    .bind(&profile_name)
    .fetch_optional(sqlx_pool)
    .await?;

    let mut transaction = sqlx_pool.begin().await?;
    let normalized = normalize_pagination_count(pagination_count);

    if let Some(existing_profile) = existing_profile {
        let mut profile_json = existing_profile
            .mm_user_profile_json
            .unwrap_or_else(|| json!({}));

        if !profile_json.is_object() {
            profile_json = json!({});
        }

        if let Some(profile_object) = profile_json.as_object_mut() {
            profile_object.insert(
                "pagination_count".to_string(),
                Value::Number(normalized.into()),
            );
        }

        sqlx::query(
            r#"
            update mm_user_profile
            set mm_user_profile_json = $2
            where mm_user_profile_guid = $1
            "#,
        )
        .bind(existing_profile.mm_user_profile_guid)
        .bind(profile_json)
        .execute(&mut *transaction)
        .await?;
    } else {
        sqlx::query(
            r#"
            insert into mm_user_profile (
                mm_user_profile_guid,
                mm_user_profile_name,
                mm_user_profile_json
            )
            values ($1, $2, $3)
            "#,
        )
        .bind(uuid::Uuid::now_v7())
        .bind(profile_name)
        .bind(json!({ "pagination_count": normalized }))
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

fn profile_name_for_user(user_id: i64) -> String {
    format!("axum_user_{}", user_id)
}
