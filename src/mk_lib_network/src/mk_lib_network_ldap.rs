// https://github.com/inejge/ldap3

use ldap3::result::Result;
use ldap3::{LdapConn, LdapConnAsync, LdapConnSettings, LdapResult, Scope, SearchEntry};

pub async fn ldap_bind(
    ldap_ip: String,
    ldap_port: String,
    ldap_bind: &str,
    ldap_secret: &str,
) -> Result<LdapResult> {
    let mut ldap = LdapConn::new(&format!("ldap://{}:{}", ldap_ip, ldap_port))?;
    let res = ldap
        .simple_bind(ldap_bind, ldap_secret)? // "cn=Manager,dc=example,dc=org"
        .success()?;
    Ok(res)
}

pub async fn ldap_unbind(mut ldap: LdapConn) -> Result<()> {
    Ok(ldap.unbind()?)
}
