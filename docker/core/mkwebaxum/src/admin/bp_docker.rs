use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_common;
use mk_lib_database;
use sqlx::postgres::PgPool;

use docker_api::models::JoinTokens;
use docker_api::models::Node;
use docker_api::models::Swarm;
use docker_api::models::SystemInfo;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_docker.html")]
struct AdminDockerTemplate<'a> {
    template_data: &'a SystemInfo,
    template_data_node_addr: &'a String,
    template_data_swarm: &'a JoinTokens,
    template_data_nodes: &'a Vec<Node>,
}

pub async fn admin_docker(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let docker_results = mk_lib_common::mk_lib_common_docker::mk_common_docker_info()
            .await
            .unwrap();
        let node_addr = docker_results.clone().swarm.unwrap().node_addr.unwrap();
        let swarm_docker_results =
            mk_lib_common::mk_lib_common_docker::mk_common_docker_swarm_inspect()
                .await
                .unwrap();
        let token = swarm_docker_results.join_tokens.unwrap();
        let node_docker_results =
            mk_lib_common::mk_lib_common_docker::mk_common_docker_swarm_nodes()
                .await
                .unwrap();
        let template = AdminDockerTemplate {
            template_data: &docker_results,
            template_data_node_addr: &node_addr,
            template_data_swarm: &token,
            template_data_nodes: &node_docker_results,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
