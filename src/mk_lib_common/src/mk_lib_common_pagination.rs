use core::fmt::Write;
use paginator::{PageItem, Paginator};
use std::error::Error;

#[path = "./mk_lib_common_internationalization.rs"]
mod mk_lib_common_internationalization;

fn build_suffix(starts_with: Option<&str>) -> String {
    match starts_with {
        Some(sw) if !sw.is_empty() => {
            // "#", spaces, etc. get encoded safely
            let enc = urlencoding::encode(sw);
            format!("?starts_with={enc}")
        }
        _ => String::new(),
    }
}

pub async fn mk_lib_common_paginate(
    total_items: i64,
    page: i64,
    base_url: String,
    starts_with: Option<&str>,
    pagination_count: i64,
) -> Result<String, Box<dyn Error>> {
    mk_lib_common_paginate_with_locale(
        total_items,
        page,
        base_url,
        starts_with,
        pagination_count,
        None,
    )
    .await
}

pub async fn mk_lib_common_paginate_with_locale(
    total_items: i64,
    page: i64,
    base_url: String,
    starts_with: Option<&str>,
    pagination_count: i64,
    locale_name: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let page_size = pagination_count.max(1);
    let total_pages = if total_items > 0 {
        (total_items + page_size - 1) / page_size
    } else {
        0
    };

    let mut pagination_html = String::new();

    if total_pages > 1 {
        pagination_html.push_str(
            r#"<nav class="mt-6 mb-6 flex justify-center" aria-label="Pagination">
<ul class="flex items-center gap-1 whitespace-nowrap text-sm">"#,
        );

        let suffix = build_suffix(starts_with);

        let paginator = Paginator::builder(total_pages as usize)
            .current_page(page.max(1) as usize)
            .build_paginator()
            .unwrap();

        for item in paginator.paginate() {
            match item {
                PageItem::Prev(p) => {
                    write!(
                        pagination_html,
                        r#"<li><a href="{url}/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
aria-label="Previous">&laquo;</a></li>"#,
                        url = base_url,
                        p = p,
                        suffix = suffix
                    )?;
                }

                PageItem::Page(p) => {
                    write!(
                        pagination_html,
                        r#"<li><a href="{url}/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200">
{label}</a></li>"#,
                        url = base_url,
                        p = p,
                        suffix = suffix,
                        label = mk_lib_common_internationalization::
                            mk_lib_common_internationalization_number_format_locale(
                                p.get() as i64,
                                locale_name
                            )
                    )?;
                }

                PageItem::CurrentPage(p) => {
                    write!(
                        pagination_html,
                        r#"<li><span
class="px-3 py-2 rounded-md bg-indigo-600 text-white font-semibold border border-indigo-600">
{p}</span></li>"#,
                        p = p
                    )?;
                }

                PageItem::Ignore => {
                    pagination_html
                        .push_str(r#"<li><span class="px-3 py-2 text-gray-400">…</span></li>"#);
                }

                PageItem::Next(p) => {
                    write!(
                        pagination_html,
                        r#"<li><a href="{url}/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
aria-label="Next">&raquo;</a></li>"#,
                        url = base_url,
                        p = p,
                        suffix = suffix
                    )?;
                }

                _ => {}
            }
        }

        pagination_html.push_str("</ul></nav>");
    }

    Ok(pagination_html)
}
