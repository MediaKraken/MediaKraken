#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use core::fmt::Write;
use paginator::{PageItem, Paginator};
use std::error::Error;

pub async fn mk_lib_common_paginate(
    total_pages: i64,
    page: i32,
    base_url: String,
) -> Result<String, Box<dyn Error>> {
    let mut pagination_html = String::new();
    if total_pages != 0 {
        pagination_html.push_str("<div>");
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
                            "<li><a href=\"{url}/{page}\">&laquo;</a></li>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::Page(page) => {
                    pagination_html
                        .write_fmt(format_args!(
                            "<li><a href=\"{url}/{page}\">{page}</a></li>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::CurrentPage(page) => {
                    pagination_html
                        .write_fmt(format_args!("<li>{page}</li>", page = page))
                        .unwrap();
                }
                PageItem::Ignore => {
                    pagination_html.push_str("<li>...</li>");
                }
                PageItem::Next(page) => {
                    // `PageItem::Next` variant is used when the `has_next` option is not set to `YesNoDepends::No`.
                    pagination_html
                        .write_fmt(format_args!(
                            "<li><a href=\"{url}/{page}\">&raquo;</a></li>",
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
        pagination_html.push_str("</div>");
    }
    Ok(pagination_html)
}
