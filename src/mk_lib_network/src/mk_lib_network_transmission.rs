// https://github.com/j0rsa/transmission-rpc

use serde::{Deserialize, Serialize};
use transmission_rpc::types::{
    BasicAuth, Id, Nothing, Result, RpcResponse, SessionClose, Torrent, TorrentAction,
    TorrentAddArgs, TorrentAddedOrDuplicate, TorrentGetField, TorrentStatus, Torrents,
};
use transmission_rpc::TransClient;

// https://docs.rs/transmission-rpc/0.4.2/transmission_rpc/types/struct.Torrent.html#structfield.added_date
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentList {
    pub mm_torrent_id: i64,           // id
    pub mm_torrent_name: String,      // name
    pub mm_torrent_status: String,    // status - transmission_rpc::types::TorrentStatus
    pub mm_torrent_size: i64,         // total_size
    pub mm_torrent_percent_done: f32, // percent_done
}

pub async fn mk_network_transmission_login() -> Result<transmission_rpc::TransClient> {
    let transmission_client = TransClient::with_auth(
        //"http://mkprod:9091/transmission/rpc".parse().unwrap(),
        "http://mkstack_transmission:9091/transmission/rpc"
            .parse()
            .unwrap(),
        BasicAuth {
            user: "admin".to_string(),
            password: "metaman".to_string(),
        },
    );
    Ok(transmission_client)
}

pub async fn mk_network_transmission_close(
    mut transmission_client: transmission_rpc::TransClient,
) -> Result<()> {
    let response: Result<RpcResponse<SessionClose>> = transmission_client.session_close().await;
    match response {
        Ok(_) => println!("Yay!"),
        Err(_) => panic!("Oh no!"),
    }
    Ok(())
}

pub async fn mk_network_transmission_add_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    file_url: String,
) -> Result<bool> {
    let add: TorrentAddArgs = TorrentAddArgs {
        // filename: Some(
        //     "https://releases.ubuntu.com/20.04/ubuntu-20.04.2.0-desktop-amd64.iso.torrent"
        //         .to_string(),
        // ),
        filename: Some(file_url),
        ..TorrentAddArgs::default()
    };
    let res: RpcResponse<TorrentAddedOrDuplicate> = transmission_client.torrent_add(add).await?;
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_list_torrents(
    mut transmission_client: transmission_rpc::TransClient,
) -> Result<Vec<TorrentList>> {
    let res: RpcResponse<Torrents<Torrent>> = transmission_client.torrent_get(None, None).await?;
    //     .torrent_get(
    //         Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
    //         Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
    // let torrent_rows: Vec<TorrentList> = select_query
    //     .map(|row: PgRow| TorrentList {
    //         mm_metadata_album_id: row.get("brainz_id"),
    //         mm_metadata_album_name: row.get("brainz_name"),
    //         mm_metadata_album_artist: row.get("brainz_artist"),
    //     })
    //     .fetch_all(sqlx_pool)
    //     .await?;
    let torrent_rows: Vec<TorrentList> = res
        .arguments
        .torrents
        .iter()
        .map(|it| TorrentList {
            mm_torrent_id: it.id.unwrap(),
            mm_torrent_name: it.name.clone().unwrap(),
            mm_torrent_status: format!("{:?}", TorrentStatus::from(it.status.unwrap())),
            mm_torrent_size: it.total_size.unwrap(),
            mm_torrent_percent_done: it.percent_done.unwrap(),
        })
        .collect();
    Ok(torrent_rows)
}

pub async fn mk_network_transmission_remove_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i64,
) -> Result<bool> {
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_remove(vec![Id::Id(torrent_id)], false)
        .await?;
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_start_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i64,
) -> Result<bool> {
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_action(TorrentAction::Start, vec![Id::Id(torrent_id)])
        .await?;
    Ok(res.is_ok())
}

pub async fn mk_network_transmission_stop_torrent(
    mut transmission_client: transmission_rpc::TransClient,
    torrent_id: i64,
) -> Result<bool> {
    let res: RpcResponse<Nothing> = transmission_client
        .torrent_action(TorrentAction::Stop, vec![Id::Id(torrent_id)])
        .await?;
    Ok(res.is_ok())
}
