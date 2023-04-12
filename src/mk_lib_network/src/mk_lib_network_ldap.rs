#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/inejge/ldap3
// ldap3 = "0.10.3"

use crate::mk_lib_logging;

use ldap3::result::Result;
use ldap3::{LdapConn, LdapConnAsync, LdapConnSettings, Scope, SearchEntry};
use serde_json::json;
use stdext::function_name;

pub async fn ldap_bind(ldap_ip: String, ldap_port: String, ldap_bind: String, ldap_secret: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut ldap = LdapConn::new(format!("ldap://{}:{}", ldap_ip, ldap_port))?;
    let _res = ldap
        .simple_bind(ldap_bind, ldap_secret)? // "cn=Manager,dc=example,dc=org"
        .success()?;
}

pub async fn ldap_unbind(ldap: LdapConn) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    ldap.unbind()?
}
