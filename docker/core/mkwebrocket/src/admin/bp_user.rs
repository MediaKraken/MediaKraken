#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::request::{self, FromRequest};
use rocket::serde::json;
use rocket::Request;
use rocket::{form::Form, get, post, routes};
use rocket::{form::*, response::Redirect, State};
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde::{Deserialize, Serialize};
use stdext::function_name;
use serde_json::json;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Serialize)]
struct TemplateAdminUserContext {
    template_data: Vec<mk_lib_database_user::DBUserList>,
    pagination_bar: String,
}

#[get("/user/<page>")]
pub async fn admin_user(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/user".to_string(),
    )
    .await
    .unwrap();
    let user_list = mk_lib_database_user::mk_lib_database_user_read(&sqlx_pool, db_offset, 30)
        .await
        .unwrap();
    Template::render(
        "bss_admin/bss_admin_user",
        &TemplateAdminUserContext {
            template_data: user_list,
            pagination_bar: pagination_html,
        },
    )
}

#[get("/user_detail/<guid>")]
pub async fn admin_user_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    Template::render(
        "bss_admin/bss_admin_user_detail",
        tera::Context::new().into_json(),
    )
}

#[post("/user_delete/<guid>")]
pub async fn admin_user_delete(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    mk_lib_database_user::mk_lib_database_user_delete(&sqlx_pool, tmp_uuid).await;
    Template::render(
        "bss_admin/bss_admin_user_delete",
        tera::Context::new().into_json(),
    )
}
