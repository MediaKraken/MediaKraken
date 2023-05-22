// https://github.com/kallekankaanpaa/rsteam

use mk_lib_logging::mk_lib_logging;
use rsteam::steam_id::{SteamID2, SteamID3};
use rsteam::steam_user::{BanData, Status};
use rsteam::{SteamClient, SteamID};
use serde_json::json;
use stdext::function_name;

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

 pub async fn steam_login(api_key: &str, vanity_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = SteamClient::with_api_key(&api_key);
    let id = client.resolve_vanity_url(&vanity_url, None).await?;
    let id_vec = vec![id.clone()];
    Ok(())
 }