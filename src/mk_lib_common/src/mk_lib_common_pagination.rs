use core::fmt::Write;
use paginator::{PageItem, Paginator};
use std::error::Error;

#[path = "./mk_lib_common_internationalization.rs"]
mod mk_lib_common_internationalization;

pub async fn mk_lib_common_paginate(
    total_pages: i64,
    page: i64,
    base_url: String,
) -> Result<String, Box<dyn Error>> {
    let mut total_pages_mut = total_pages;
    if total_pages_mut > 0 {
        total_pages_mut /= 30;
    }

    let mut pagination_html = String::new();

    if total_pages_mut != 0 {
        pagination_html.push_str(
            r#"<nav class="flex justify-center mt-6" aria-label="Pagination">
<ul class="inline-flex items-center gap-1 text-sm">"#,
        );

        let paginator = Paginator::builder(total_pages_mut as usize)
            .current_page(page as usize)
            .build_paginator()
            .unwrap();

        for page_item in paginator.paginate() {
            match page_item {
                PageItem::Prev(page) => {
                    write!(
                        pagination_html,
                        r#"<li>
<a href="{url}/{page}"
   class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
   aria-label="Previous">
  <span aria-hidden="true">&laquo;</span>
  <span class="sr-only">Previous</span>
</a>
</li>"#,
                        url = base_url,
                        page = page
                    )?;
                }

                PageItem::Page(page) => {
                    write!(
                        pagination_html,
                        r#"<li>
<a href="{url}/{page}"
   class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200">
  {page_format}
</a>
</li>"#,
                        url = base_url,
                        page = page,
                        page_format = mk_lib_common_internationalization::
                            mk_lib_common_internationalization_number_format(
                                page.get() as i64
                            )?
                    )?;
                }

                PageItem::CurrentPage(page) => {
                    write!(
                        pagination_html,
                        r#"<li>
<span class="px-3 py-2 rounded-md bg-indigo-600 text-white font-semibold border border-indigo-600">
  {page}
  <span class="sr-only">(current)</span>
</span>
</li>"#,
                        page = page
                    )?;
                }

                PageItem::Ignore => {
                    pagination_html.push_str(
                        r#"<li>
<span class="px-3 py-2 text-gray-400">…</span>
</li>"#,
                    );
                }

                PageItem::Next(page) => {
                    write!(
                        pagination_html,
                        r#"<li>
<a href="{url}/{page}"
   class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
   aria-label="Next">
  <span aria-hidden="true">&raquo;</span>
  <span class="sr-only">Next</span>
</a>
</li>"#,
                        url = base_url,
                        page = page
                    )?;
                }

                _ => {}
            }
        }

        pagination_html.push_str("</ul></nav>");
    }

    Ok(pagination_html)
}
