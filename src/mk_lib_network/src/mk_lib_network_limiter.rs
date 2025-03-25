//use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use phf::phf_map;

// https://docs.rs/governor/0.6.3/governor/struct.Quota.html

// pub async fn mk_network_rate_limiter() {
//     // Allow 50 units per second
//     let lim = RateLimiter::direct(Quota::per_second(nonzero!(50u32)));
//     //assert_eq!(Ok(()), lim.check());
//     //Ok(lim)
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

 // Requests, Time in Seconds, Per Day
pub static API_LIMIT: phf::Map<&'static str, (u64, u64, u64)> = phf_map! {
    "anidb" => (1, 4, u64::MAX),  // A Client MUST NOT send more than one packet
    // every four seconds over an extended amount of time. 4-16-2016)
    // "barcodelookup" => (0, 0, u64::MAX),
    "barcodespider" => (u64::MAX, u64::MAX, 100),
    "chart_lyrics" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016 just says don't abuse
    "comicvine" => (1, 1, u64::MAX),  // 4-16-2016
    "discogs" => (240, 60, u64::MAX),  // 1-16-2017
    "flickr" => (3000, 60, u64::MAX), // 5/4/2023
    "giantbomb" => (1, 1, u64::MAX),  // 10-18-2020) 1 per second or hit wall hard
    // "goupc" => (0, 0, u64::MAX),
    "imdb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    "imvdb" => (1000, 60, u64::MAX),  // 1000 per minute 6/30/2016
    "isbndb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 1/01/2017
    "lastfm" => (5, 1, u64::MAX),  // five per second 11/8/2016
    "musicbrainz" => (1, 1, u64::MAX),  // 1 per second 11/11/2017
    "omdb" => (20, 1, u64::MAX),  // 7/29/2016 says 20 concurrent connections
    "openlibrary" => (100, 300, u64::MAX),  // 1/14/2017 100 every 5 minutes
    "pitchfork" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    "pornhub" => (1, 1, u64::MAX),  // since I'm scraping
    "televisiontunes" => (1, 1, u64::MAX),  // since I'm scraping
    "theaudiodb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    "thegamesdb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    //  "thelogodb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    "themoviedb" => (u64::MAX, 1, u64::MAX),  // limit has been lifted 4/3/2022
    "thesportsdb" => (u64::MAX, 1, u64::MAX),  // no mention of limits 7/29/2016
    //  "thetvdb" => (u64::MAX, 1, u64::MAX),  // no mention of limits besides play nice 4-16-2016
    "tv_intros" => (1, 1, u64::MAX),  // since I'm scraping
    //  "tvmaze" => (20, 10, u64::MAX),  // 20 every 10 6-11-2017
    "twitch" => (1, 1, u64::MAX),  // 12-10-2017
    "upcitemdb" => (6, 60, 100), // 05-17-2024 - Ex. for plan FREE, your application is limited up to 6 requests per minute. 
    // Requests sent faster than that will be declined with HTTP status 429. In order to regain the access, 
    // your application has to wait for the next window period. Think of it the same as the daily limits, but with a much smaller window.
    "Z" => (u64::MAX, u64::MAX, u64::MAX),  // catch all for limiter api program
};
