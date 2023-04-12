#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/kallekankaanpaa/rsteam
// rsteam = "0.1.3"

use rsteam::steam_id::{SteamID2, SteamID3};
use rsteam::steam_user::{BanData, Status};
use rsteam::{SteamClient, SteamID};
use serde_json::json;
use stdext::function_name;

use crate::mk_lib_logging;

/*
ISteamUser
ResolveVanityURL
GetPlayerSummaries
GetPlayerBans
GetFriendList
GetUserGroupList
ISteamUserStats
GetGlobalAchievementPercentagesForApp
GetNumberOfCurrentPlayers
GetUserStatsForGame
IPlayerService
GetBadges
GetCommunityBadgeProcess
GetOwnedGames
GetRecentlyPlayedGames
GetSteamLevel
IsPlayingSharedGame
ISteamApps
GetAppList
ISteamNews
GetNewsForApp
 */
