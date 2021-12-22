/*
# https://github.com/MediaKraken-Dependancies/pylast
import pylast


class CommonMetadataLastFM:
    """
    Class for interfacing with lastfm
    """

    def __init__(self, option_config_json):
        self.lastfm_inst = pylast.LastFMNetwork(api_key=option_config_json['LastFM']['api_key'],
                                                api_secret=option_config_json['LastFM'][
                                                    'api_secret'],
                                                username=option_config_json['LastFM']['username'],
                                                password_hash=option_config_json['LastFM'][
                                                    'password'])

        # # Now you can use that object everywhere
        # artist = network.get_artist("System of a Down")
        # artist.shout("<3")
        #
        #
        # track = network.get_track("Iron Maiden", "The Nomad")
        # track.love()
        # track.add_tags(("awesome", "favorite"))
        #
        # # Type help(pylast.LastFMNetwork) or help(pylast) in a Python interpreter
        # # to get more help about anything and see examples of how it works

 */