#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use phf::phf_map;
use std::num::NonZeroU32;

// https://docs.rs/governor/0.3.2/governor/struct.Quota.html
// pub fn mk_network_rate_limiter {
//     let mut lim = RateLimiter::direct(Quota::per_second(nonzero!(50u32))); // Allow 50 units per second
//     assert_eq!(Ok(()), lim.check());
// }
/*
let lim = RateLimiter::direct(Quota::per_second(nonzero!(10u32)));
    // exhaust the limiter:
    loop {
        if lim.check().is_err() {
            break;
        }
    }
    block_on(lim.until_ready());
 */

pub static API_LIMIT: phf::Map<&'static str, i16, i16> = phf_map! {
    "anidb" => 1, 4,  // A Client MUST NOT send more than one packet
    // every four seconds over an extended amount of time. 4-16-2016)
    "chart_lyrics" => 9999, 1,  // no mention of limits 7/29/2016 just says don't abuse
    "comicvine" => 1, 1,  // 4-16-2016)
    "discogs" => 240, 60,  // 1-16-2017)
    "giantbomb" => 1, 1,  // 10-18-2020) 1 per second or hit wall hard
    "imdb" => 9999, 1,  // no mention of limits 7/29/2016
    "imvdb" => 1000, 60,  // 1000 per minute 6/30/2016)
    "isbndb" => 9999, 1,  // no mention of limits 1/01/2017
    "lastfm" => 5, 1,  // five per second 11/8/2016)
    "musicbrainz" => 1, 1,  // 1 per second 11/11/2017)
    "omdb" => 20, 1,  // 7/29/2016 says 20 concurrent connections
    "openlibrary" => 100, 300,  // 1/14/2017 100 every 5 minutes
    "pitchfork" => 9999, 1,  // no mention of limits 7/29/2016
    "pornhub" => 1, 1,  // since I'm scraping
    "televisiontunes" => 1, 1,  // since I'm scraping
    "theaudiodb" => 9999, 1,  // no mention of limits 7/29/2016
    "thegamesdb" => 9999, 1,  // no mention of limits 7/29/2016
    //  "thelogodb" => 9999, 1,  // no mention of limits 7/29/2016
    "themoviedb" => 9999, 1,  // limit has been lifted 4/3/2022
    "thesportsdb" => 9999, 1,  // no mention of limits 7/29/2016
    //  "thetvdb" => 9999, 1,  // no mention of limits besides play nice 4-16-2016)
    "tv_intros" => 1, 1,  // since I'm scraping
    //  "tvmaze" => 20, 10,  // 20 every 10 6-11-2017)
    "twitch" => 1, 1,  // 12-10-2017)
    "Z" => None, None,  // catch all for limiter api program
};
