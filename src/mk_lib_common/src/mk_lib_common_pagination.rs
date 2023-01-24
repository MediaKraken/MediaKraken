#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use core::fmt::Write;
use paginator::{PageItem, Paginator};
use std::error::Error;
use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_common_paginate(
    total_pages: i64,
    page: i32,
    base_url: String,
) -> Result<String, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut pagination_html = String::new();
    if total_pages != 0 {
        pagination_html.push_str("<div><table><tr>");
        let paginator = Paginator::builder(total_pages as usize)
            .current_page(page as usize)
            .build_paginator()
            .unwrap();
        for page_item in paginator.paginate() {
            match page_item {
                PageItem::Prev(page) => {
                    // `PageItem::Prev` variant is used when the `has_prev` option is not set to `YesNoDepends::No`.
                    pagination_html
                        .write_fmt(format_args!(
                            "<td><a href=\"{url}/{page}\">&laquo;</a></td>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::Page(page) => {
                    pagination_html
                        .write_fmt(format_args!(
                            "<td><a href=\"{url}/{page}\">{page}</a></td>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::CurrentPage(page) => {
                    pagination_html
                        .write_fmt(format_args!("<td>{page}</td>", page = page))
                        .unwrap();
                }
                PageItem::Ignore => {
                    pagination_html.push_str("<td>...</td>");
                }
                PageItem::Next(page) => {
                    // `PageItem::Next` variant is used when the `has_next` option is not set to `YesNoDepends::No`.
                    pagination_html
                        .write_fmt(format_args!(
                            "<td><a href=\"{url}/{page}\">&raquo;</a></td>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                _ => {
                    // `PageItem::ReservedPrev` or `PageItem::ReservedNext` variant is used only when the `has_prev` option or the `has_next` option is set to `YesNoDepends::Yes`.
                }
            }
        }
        pagination_html.push_str("</tr></table></div>");
    }
    Ok(pagination_html)
}
