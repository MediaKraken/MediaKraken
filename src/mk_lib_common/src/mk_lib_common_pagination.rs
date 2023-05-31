use mk_lib_logging::mk_lib_logging;
use core::fmt::Write;
use paginator::{PageItem, Paginator};
use serde_json::json;
use std::error::Error;
use stdext::function_name;

pub async fn mk_lib_common_paginate(
    total_pages: i64,
    page: i64,
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
    let mut total_pages_mut = total_pages;
    if total_pages_mut > 0 {
        total_pages_mut = total_pages_mut / 30;
    }
    let mut pagination_html = String::new();
    if total_pages_mut != 0 {
        pagination_html.push_str("<div><ul class=\"pagination\">");
        let paginator = Paginator::builder(total_pages_mut as usize)
            .current_page(page as usize)
            .build_paginator()
            .unwrap();
        for page_item in paginator.paginate() {
            match page_item {
                PageItem::Prev(page) => {
                    // `PageItem::Prev` variant is used when the `has_prev` option is not set to `YesNoDepends::No`.
                    pagination_html
                        .write_fmt(format_args!(
                            "<li class=\"page-item\"><a class=\"page-link\" href=\"{url}/{page}\" aria-label=\"Previous\"><span aria-hidden=\"true\">&laquo;</span><span class=\"sr-only\">Previous</span></a></li>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::Page(page) => {
                    pagination_html
                        .write_fmt(format_args!(
                            "<li class=\"page-item\"><a class=\"page-link\" href=\"{url}/{page}\">{page}</a></li>",
                            url = base_url,
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::CurrentPage(page) => {
                    pagination_html
                        .write_fmt(format_args!(
                            "<li class=\"page-item active\"><span class=\"page-link\">{page}<span class=\"sr-only\">(current)</span></span></li>",
                            page = page
                        ))
                        .unwrap();
                }
                PageItem::Ignore => {
                    pagination_html.push_str("<li class=\"page-item\">...</li>");
                }
                PageItem::Next(page) => {
                    // `PageItem::Next` variant is used when the `has_next` option is not set to `YesNoDepends::No`.
                    pagination_html
                        .write_fmt(format_args!("<li class=\"page-item\"><a class=\"page-link\" href=\"{url}/{page}\" aria-label=\"Next\"><span aria-hidden=\"true\">&raquo;</span><span class=\"sr-only\">Next</span></a></li>",
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
        pagination_html.push_str("</ul></div>");
    }
    Ok(pagination_html)
}
