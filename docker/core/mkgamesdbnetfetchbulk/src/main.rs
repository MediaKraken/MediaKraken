/*

# open the database
option_config_json, db_connection = common_config_ini.com_config_read()

GAMESDB_INST = metadata_provider_thegamesdb.CommonMetadataGamesDB()

# grab and insert all platforms
for platform in \
        list(GAMESDB_INST.com_meta_gamesdb_platform_list()['Data']['Platforms'].items())[0]:
    if platform != 'Platform':
        for game_systems in platform:
            print(game_systems, flush=True)
            if db_connection.db_meta_games_system_guid_by_short_name(game_systems['name']) is None:
                # fetch platform info
                platform_json = GAMESDB_INST.com_meta_gamesdb_platform_by_id(game_systems['id'])
                # store record
                try:
                    system_alias = game_systems['alias']
                except KeyError:
                    system_alias = None
                db_connection.db_meta_games_system_insert(game_systems['id'],
                                                          game_systems['name'],
                                                          system_alias,
                                                          json.dumps(platform_json))
                db_connection.db_commit()


# commit all changes
db_connection.db_commit()

# close DB
db_connection.db_close()

 */