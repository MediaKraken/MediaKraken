use crate::axum_custom_filters::filters;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use crate::mk_lib_database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{FromRow, Row};
use axum::extract::State;
use crate::AppState;

// TODO initial page load for media type

// TODO media details