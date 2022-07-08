#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/inejge/ldap3
// ldap3 = "0.10.3"

use ldap3::result::Result;
use ldap3::{LdapConn, LdapConnAsync, LdapConnSettings, Scope, SearchEntry};

pub async fn ldap_bind(ldap_ip: String, ldap_port: String, ldap_bind: String, ldap_secret: String) {
    let mut ldap = LdapConn::new(format!("ldap://{}:{}", ldap_ip, ldap_port))?;
    let _res = ldap
        .simple_bind(ldap_bind, ldap_secret)? // "cn=Manager,dc=example,dc=org"
        .success()?;
}

pub async fn ldap_unbind(ldap: LdapConn) {
    ldap.unbind()?
}
