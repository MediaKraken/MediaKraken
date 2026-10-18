//! Rust API for Steam Web API calls.
//!
//! - Typed endpoints (ISteamUser, ISteamUserStats, IPlayerService, ISteamApps,
//!   ISteamNews and the legacy group endpoints) come from [`rsteam`]; they are
//!   re-exported here and available directly on [`SteamApi`].
//! - Any other `api.steampowered.com` endpoint can be called with
//!   [`SteamApi::call`] / [`SteamApi::call_typed`].

pub mod mk_lib_steam_api;
pub mod mk_lib_steam_error;

pub use mk_lib_steam_api::SteamApi;
pub use mk_lib_steam_error::{Result, SteamError};

pub use rsteam::{
    SteamClient, SteamID, legacy, player_service, steam_apps, steam_news, steam_user,
    steam_user_stats,
};
