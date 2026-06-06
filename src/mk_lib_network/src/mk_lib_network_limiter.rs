//use governor::{Quota, RateLimiter};
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
    "themoviedb" => (45, 1, u64::MAX),  // 03/30/2025 hard scrape limits
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_limit_contains_anidb() {
        let limit = API_LIMIT.get("anidb").copied().unwrap();
        assert_eq!(limit, (1, 4, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_barcodespider() {
        let limit = API_LIMIT.get("barcodespider").copied().unwrap();
        assert_eq!(limit.2, 100); // daily cap of 100
    }

    #[test]
    fn test_api_limit_contains_comicvine() {
        let limit = API_LIMIT.get("comicvine").copied().unwrap();
        assert_eq!(limit, (1, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_flickr() {
        let limit = API_LIMIT.get("flickr").copied().unwrap();
        assert_eq!(limit, (3000, 60, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_giantbomb() {
        let limit = API_LIMIT.get("giantbomb").copied().unwrap();
        assert_eq!(limit, (1, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_lastfm() {
        let limit = API_LIMIT.get("lastfm").copied().unwrap();
        assert_eq!(limit, (5, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_musicbrainz() {
        let limit = API_LIMIT.get("musicbrainz").copied().unwrap();
        assert_eq!(limit, (1, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_themoviedb() {
        let limit = API_LIMIT.get("themoviedb").copied().unwrap();
        assert_eq!(limit, (45, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_upcitemdb() {
        let limit = API_LIMIT.get("upcitemdb").copied().unwrap();
        assert_eq!(limit, (6, 60, 100));
    }

    #[test]
    fn test_api_limit_contains_catch_all() {
        let limit = API_LIMIT.get("Z").copied().unwrap();
        assert_eq!(limit, (u64::MAX, u64::MAX, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_imdb() {
        let limit = API_LIMIT.get("imdb").copied().unwrap();
        assert_eq!(limit.0, u64::MAX);
    }

    #[test]
    fn test_api_limit_contains_themoviedb_limits() {
        let limit = API_LIMIT.get("themoviedb").copied().unwrap();
        assert!(limit.0 > 0);
        assert!(limit.0 <= 45);
        assert_eq!(limit.1, 1);
    }

    #[test]
    fn test_api_limit_contains_discogs() {
        let limit = API_LIMIT.get("discogs").copied().unwrap();
        assert_eq!(limit, (240, 60, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_imvdb() {
        let limit = API_LIMIT.get("imvdb").copied().unwrap();
        assert_eq!(limit, (1000, 60, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_openlibrary() {
        let limit = API_LIMIT.get("openlibrary").copied().unwrap();
        assert_eq!(limit, (100, 300, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_omdb() {
        let limit = API_LIMIT.get("omdb").copied().unwrap();
        assert_eq!(limit, (20, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_twitch() {
        let limit = API_LIMIT.get("twitch").copied().unwrap();
        assert_eq!(limit, (1, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_contains_tv_intros() {
        let limit = API_LIMIT.get("tv_intros").copied().unwrap();
        assert_eq!(limit, (1, 1, u64::MAX));
    }

    #[test]
    fn test_api_limit_catch_all_has_max_values() {
        let catch_all = API_LIMIT.get("Z").copied().unwrap();
        assert_eq!(catch_all.0, u64::MAX);
        assert_eq!(catch_all.1, u64::MAX);
        assert_eq!(catch_all.2, u64::MAX);
    }

    #[test]
    fn test_api_limit_all_have_positive_requests() {
        for (key, &(req, _, _)) in API_LIMIT.iter() {
            if key != &"Z" {
                assert!(req > 0, "API limit for {} has zero requests", key);
            }
        }
    }

    #[test]
    fn test_api_limit_all_have_positive_time() {
        for (key, &(_, time, _)) in API_LIMIT.iter() {
            if key != &"Z" {
                assert!(time > 0, "API limit for {} has zero time window", key);
            }
        }
    }

    #[test]
    fn test_api_limit_contains_theaudiodb() {
        assert!(API_LIMIT.contains_key("theaudiodb"));
    }

    #[test]
    fn test_api_limit_contains_thegamesdb() {
        assert!(API_LIMIT.contains_key("thegamesdb"));
    }

    #[test]
    fn test_api_limit_contains_thesportsdb() {
        assert!(API_LIMIT.contains_key("thesportsdb"));
    }
}
