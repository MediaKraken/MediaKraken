use paginator::{Paginator, PageItem};
use core::fmt::Write;
use std::error::Error;

pub async fn mk_lib_common_paginate(total_pages: i32, page: i8)
                                    -> Result<String, Box<dyn Error>>{
    let paginator = Paginator::builder(total_pages as usize).current_page(page as usize).build_paginator().unwrap();
    let mut pagination_html = String::new();
    for page_item in paginator.paginate() {
        match page_item {
            PageItem::Prev(page) => {
                // `PageItem::Prev` variant is used when the `has_prev` option is not set to `YesNoDepends::No`.
                pagination_html.write_fmt(format_args!("<li><a href=\"/page/{page}\">&laquo;</a></li>", page = page)).unwrap();
            }
            PageItem::Page(page) => {
                pagination_html.write_fmt(format_args!("<li><a href=\"/page/{page}\">{page}</a></li>", page = page)).unwrap();
            }
            PageItem::CurrentPage(page) => {
                pagination_html.write_fmt(format_args!("<li>{page}</li>", page = page)).unwrap();
            }
            PageItem::Ignore => {
                pagination_html.push_str("<li>...</li>");
            }
            PageItem::Next(page) => {
                // `PageItem::Next` variant is used when the `has_next` option is not set to `YesNoDepends::No`.
                pagination_html.write_fmt(format_args!("<li><a href=\"/page/{page}\">&raquo;</a></li>", page = page)).unwrap();
            }
            _ => {
                // `PageItem::ReservedPrev` or `PageItem::ReservedNext` variant is used only when the `has_prev` option or the `has_next` option is set to `YesNoDepends::Yes`.
            }
        }
    }
    Ok(pagination_html)
}