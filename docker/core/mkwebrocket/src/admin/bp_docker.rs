#![cfg_attr(debug_assertions, allow(dead_code))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_docker.rs"]
mod mk_lib_common_docker;

#[get("/docker")]
pub async fn admin_docker(user: AdminUser) -> Template {
    let docker_results = mk_lib_common_docker::mk_common_docker_info().await.unwrap();
    Template::render(
        "bss_admin/bss_admin_docker",
        tera::Context::new().into_json(),
    )
}

/*

@blueprint_admin_docker.route("/admin_docker_stat")
@common_global.jinja_template.template('bss_admin/bss_admin_docker.html')
@common_global.auth.login_required
pub async fn url_bp_admin_docker_stat(request):
    """
    Docker statistics including swarm
    """
    docker_inst = common_docker.CommonDocker()
    # it returns a dict, not a json
    docker_info = docker_inst.com_docker_info()
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'Docker info': docker_info})
    if 'Managers' not in docker_info['Swarm'] or docker_info['Swarm']['Managers'] == 0:
        docker_swarm = "Cluster not active"
        docker_nodes = None
    else:
        docker_swarm = docker_inst.com_docker_swarm_inspect()['JoinTokens']['Worker']
        docker_nodes = docker_inst.com_docker_node_list()
    return {
        'data_host': docker_info,
        'data_swam': docker_swarm,
        'data_nodes': docker_nodes,
    }

 */
