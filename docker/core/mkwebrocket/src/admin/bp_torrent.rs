#![cfg_attr(debug_assertions, allow(dead_code))]

use bytesize::ByteSize;
use core::fmt::Write;
use paginator::{PageItem, Paginator};
use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;
use transmission_rpc::types::{
    FreeSpace, Id, Nothing, Result, RpcResponse, SessionClose, Torrent, TorrentAction,
    TorrentAddArgs, TorrentAddedOrDuplicate, TorrentGetField, Torrents,
};
use transmission_rpc::TransClient;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_network_transmission.rs"]
mod mk_lib_network_transmission;

#[get("/torrent")]
pub async fn admin_torrent(user: AdminUser) -> Template {
    let mut transmission_client = TransClient::new("mkstack_transmission".parse().unwrap());

    let res: RpcResponse<Torrents<Torrent>> =
        transmission_client.torrent_get(None, None).await.unwrap();
    let names: Vec<&String> = res
        .arguments
        .torrents
        .iter()
        .map(|it| it.name.as_ref().unwrap())
        .collect();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "names": names }))
            .await
            .unwrap();
    }

    let res1: RpcResponse<Torrents<Torrent>> = transmission_client
        .torrent_get(
            Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
            Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
        )
        .await
        .unwrap();
    let first_three: Vec<String> = res1
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{}. {}",
                &it.id.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "first_three": first_three }),
        )
        .await
        .unwrap();
    }

    let res2: RpcResponse<Torrents<Torrent>> = transmission_client
        .torrent_get(
            Some(vec![
                TorrentGetField::Id,
                TorrentGetField::HashString,
                TorrentGetField::Name,
            ]),
            Some(vec![Id::Hash(String::from(
                "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
            ))]),
        )
        .await
        .unwrap();
    let info: Vec<String> = res2
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{:5}. {:^45} {}",
                &it.id.as_ref().unwrap(),
                &it.hash_string.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();

    let response: Result<RpcResponse<SessionClose>> = transmission_client.session_close().await;
    Template::render(
        "bss_admin/bss_admin_torrent",
        tera::Context::new().into_json(),
    )
}

/*

@blueprint_admin_torrent.route('/admin_torrent_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_torrent_delete(request):
    """
    Delete torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_torrent.route('/admin_torrent_edit', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_torrent_edit(request):
    """
    Edit a torrent
    """
    # await request.app.db_functions.db_transmission_delete(request.form['id'], db_connection)
    return json.dumps({'status': 'OK'})

 */
