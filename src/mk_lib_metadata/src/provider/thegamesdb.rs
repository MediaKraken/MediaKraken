#[path = "mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataGamesDB:
    """
    Class for interfacing with theGamesDB
    """

    def __init__(self, option_config_json):
        self.BASE_URL = 'http://thegamesdb.net/api/'
        self.httpheaders = {'Accept': 'application/json',
                            'Content-Type': 'application/x-www-form-urlencoded'}

    async def com_meta_gamesdb_platform_list(self):
        """
        Get platform list
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetPlatformsList.php',
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_platform_by_id(self, platform_id):
        """
        Platform info by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetPlatform.php?id=%s' % platform_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_by_name(self, game_name):
        """
        # 'mega man'
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetGamesList.php?name=%s'
                                                    % game_name.replace(' ', '%20'),
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_by_id(self, game_id):
        """
        # game by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetGamesList.php?id=%s' % game_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_art_by_id(self, game_id):
        """
        # game by id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetArt.php?id=%s' % game_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_by_platform_id(self, platform_id):
        """
        Games by platform id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'GetPlatformGames.php?platform=%s'
                                                    % platform_id,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_by_platform_name(self, platform_name):
        """
        Games by platform id
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'PlatformGames.php?platform=%s'
                                                    % platform_name,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_games_updated_seconds(self, update_time):
        """
        Games updated in last n seconds
        """
        async with httpx.AsyncClient() as client:
            return xmltodict.parse(await client.get(self.BASE_URL + 'Updates.php?time=%s' % update_time,
                                                    headers=self.httpheaders,
                                                    timeout=3.05))

    async def com_meta_gamesdb_bulk_json(self, update_time):
        """
        Grab the json database dump and process
        """
        async with httpx.AsyncClient() as client:
            for game_row in xmltodict.parse(await client.get(
                    'https://cdn.thegamesdb.net/json/database-latest.json', timeout=3.05))['games']:
                pass

 */