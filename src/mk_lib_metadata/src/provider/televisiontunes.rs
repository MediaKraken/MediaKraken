#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

// https://www.televisiontunes.com/Saving_Grace.html
// so, replace space with _ and go to url?
// <p class="media480 m_down mm"><a id="download_song" href="/song/download/4601">Download Saving Grace </a></p>
// pull the href and add to base url, and download

pub async fn provider_televisiontunes_theme_fetch(pool: &sqlx::PgPool, tv_show_name: String) {}
