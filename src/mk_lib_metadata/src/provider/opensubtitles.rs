#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://forum.opensubtitles.org/viewtopic.php?f=8&t=14563

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

/*

class CommonMetadataOpenSubtitles:
    """
    Class for interfacing with Opensubtitles
    """

    def __init__(self, user_name, user_password):
        self.opensubtitles_inst = OpenSubtitles()
        self.token = self.opensubtitles_inst.login(user_name, user_password)

    async def com_meta_opensub_search(self, file_name):
        f = File(file_name)
        return self.opensubtitles_inst.search_subtitles([{'sublanguageid': 'all',
                                                          'moviehash': f.get_hash(),
                                                          'moviebytesize': f.size}])

    async def com_meta_opensub_ping(self):
        self.opensubtitles_inst.no_operation()

    async def com_meta_opensub_logoff(self):
        self.opensubtitles_inst.logout()

 */