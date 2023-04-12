#![cfg_attr(debug_assertions, allow(dead_code))]

// https://api.thegamesdb.net/
// https://cdn.thegamesdb.net/json/database-latest.json - db dump

use serde_json::json;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

use crate::mk_lib_network;

/*
{"code":200,"status":"Success","last_edit_id":922629,"data":{"count":93323,"games":[{"id":1,"game_title":"Halo: Combat Evolved","release_date":"2003-09-30","platform":1,"region_id":1,"country_id":0,"overview":"In Halo&#039;s twenty-sixth century setting, the player assumes the role of the Master Chief, a cybernetically enhanced super-soldier. The player is accompanied by Cortana, an artificial intelligence who occupies the Master Chief&#039;s neural interface. Players battle various aliens on foot and in vehicles as they attempt to uncover the secrets of the eponymous Halo, a ring-shaped artificial planet.","youtube":"dR3Hm8scbEw","players":16,"coop":"Yes","rating":"M - Mature 17+","developers":[1389,3423],"genres":[1,8],"publishers":[1],"alternates":null,"uids":null,"hashes":null},{"id":2,"game_title":"Crysis","release_date":"2007-11-13","platform":1,"region_id":0,"country_id":0,"overview":"From the makers of Far Cry, Crysis offers FPS fans the best-looking, most highly-evolving gameplay, requiring the player to use adaptive tactics and total customization of weapons and armor to survive in dynamic, hostile environments including Zero-G. \r\n\r\nEarth, 2019. A team of US scientists makes a frightening discovery on an island in the South China Sea. All contact with the team is lost when the North Korean Government quickly seals off the area. The United States responds by dispatching an elite team of Delta Force Operators to recon the situation. As tension rises between the two nations, a massive alien ship reveals itself in the middle of the island. The ship generates an immense force sphere that freezes a vast portion of the island and drastically alters the global weather system. Now the US and North Korea must join forces to battle the alien menace. With hope rapidly fading, you must fight epic battles through tropical jungle, frozen landscapes, and finally into the heart of the alien ship itself for the ultimate Zero G showdown.","youtube":"i3vO01xQ-DM","players":4,"coop":"No","rating":"M - Mature 17+","developers":[1970],"genres":[8],"publishers":[2],"alternates":null,"uids":null,"hashes":null}
 */

pub async fn thegamesdb_database_fetch() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(
        "https://cdn.thegamesdb.net/json/database-latest.json".to_string(),
    )
    .await
    .unwrap();
}

pub async fn thegamesdb_platforms_read(api_key: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value =
        mk_lib_network::mk_data_from_url_to_json(format!("https://api.thegamesdb.net/v1/Platforms?apikey={}?fields=icon,console,controller,developer,manufacturer,media,cpu,memory,graphics,sound,maxcontrollers,display,overview,youtube", api_key)).await.unwrap();
}

pub async fn thegamesdb_games_updated(api_key: String, edit_id: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.thegamesdb.net/v1/Games/Updates?apikey={}?last_edit_id={}",
        api_key, edit_id
    ))
    .await
    .unwrap();
}

pub async fn thegamesdb_genre_read(api_key: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.thegamesdb.net/v1/Genres?apikey={}",
        api_key
    ))
    .await
    .unwrap();
}

pub async fn thegamesdb_developers_read(api_key: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.thegamesdb.net/v1/Developers?apikey={}",
        api_key
    ))
    .await
    .unwrap();
}

pub async fn thegamesdb_publishers_read(api_key: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let json_data: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.thegamesdb.net/v1/Publishers?apikey={}",
        api_key
    ))
    .await
    .unwrap();
}
/*
class CommonMetadataGamesDB:
    """
    Class for interfacing with theGamesDB
    """

    pub async fn com_meta_gamesdb_platform_by_id(self, platform_id):
        """
        Platform info by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetPlatform.php?id=%s' % platform_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    pub async fn com_meta_gamesdb_games_by_name(self, game_name):
        """
        # 'mega man'
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetGamesList.php?name=%s'
                                                    % game_name.replace(' ', '%20'),
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    pub async fn com_meta_gamesdb_games_by_id(self, game_id):
        """
        # game by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetGamesList.php?id=%s' % game_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    pub async fn com_meta_gamesdb_games_art_by_id(self, game_id):
        """
        # game by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetArt.php?id=%s' % game_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    pub async fn com_meta_gamesdb_games_by_platform_id(self, platform_id):
        """
        Games by platform id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetPlatformGames.php?platform=%s'
                                                    % platform_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

 */
