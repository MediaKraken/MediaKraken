#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::error::Error;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

# https://github.com/MediaKraken-Dep/pornhub-api

class CommonMetadataPornhub:
    """
    Class for interfacing with pornhub
    """

    def __init__(self, proxy_ip=None, proxy_port=None):
        if proxy_ip is None:
            self.pornhub_inst = pornhub.PornHub()
        else:
            self.pornhub_inst = pornhub.PornHub(proxy_ip, proxy_port)

    pub async fn com_meta_pornhub_stars(self, star_limit):
        return self.pornhub_inst.getStars(star_limit)

    pub async fn com_meta_pornhub_search(self, search_keywords, quantity, page):
        self.pornhub_inst = pornhub.PornHub(search_keywords)
        return self.pornhub_inst.getVideos(quantity, page=page)

 */
