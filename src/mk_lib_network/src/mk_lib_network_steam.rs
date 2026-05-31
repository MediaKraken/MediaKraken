// https://github.com/kallekankaanpaa/rsteam

use rsteam::SteamClient;

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

pub async fn steam_login(
    api_key: &str,
    vanity_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = SteamClient::with_api_key(api_key);
    let id = client.resolve_vanity_url(vanity_url, None).await?;
    let _id_vec = [id];
    Ok(())
}
