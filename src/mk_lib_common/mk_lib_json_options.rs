#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct MediaKrakenOptions {
    street: String,
    city: String,
}

/*
{
  "API": {
    "anidb": null,
    "imvdb": null,
    "google": "AIzaSyCwMkNYp8E4H19BDzlM7-IDkNCQtw0R9lY",
    "isbndb": "25C8IT4I",
    "tvmaze": "mknotneeded",
    "thetvdb": "147CB43DCA8B61B7",
    "shoutcast": null,
    "thelogodb": null,
    "soundcloud": null,
    "themoviedb": "f72118d1e84b8a1438935972a9c37cac",
    "globalcache": null,
    "musicbrainz": null,
    "thesportsdb": "4352761817344",
    "opensubtitles": null,
    "openweathermap": "575b4ae4615e4e2a4c34fb9defa17ceb",
    "rottentomatoes": "f4tnu5dn9r7f28gjth3ftqaj"
  },
  "MAME": {
    "Version": 240
  },
  "User": {
    "Password Lock": null,
    "Activity Purge": null
  },
  "Cloud": {},
  "Trakt": {
    "ApiKey": null,
    "ClientID": null,
    "SecretKey": null
  },
  "Backup": {
    "Interval": 0,
    "BackupType": "local"
  },
  "Docker": {
    "Nodes": 0,
    "SwarmID": null,
    "Instances": 0
  },
  "LastFM": {
    "api_key": null,
    "password": null,
    "username": null,
    "api_secret": null
  },
  "Twitch": {
    "OAuth": null,
    "ClientID": null
  },
  "Account": {
    "ScheduleDirect": {
      "User": null,
      "Password": null
    }
  },
  "Metadata": {
    "Trailer": {
      "Clip": false,
      "Behind": false,
      "Carpool": false,
      "Trailer": false,
      "Featurette": false
    },
    "DL Subtitle": false,
    "MusicBrainz": {
      "Host": null,
      "Port": 5000,
      "User": null,
      "Password": null
    },
    "MetadataImageLocal": false
  },
  "Transmission": {
    "Host": null,
    "Port": 9091,
    "Password": "metaman",
    "Username": "spootdev"
  },
  "Docker Instances": {
    "elk": false,
    "smtp": false,
    "mumble": false,
    "pgadmin": false,
    "portainer": false,
    "teamspeak": false,
    "wireshark": false,
    "musicbrainz": false,
    "transmission": false
  },
  "MediaKrakenServer": {
    "MOTD": null,
    "Maintenance": null,
    "Server Name": "MediaKraken",
    "MaxResumePct": 5
  }
} */
