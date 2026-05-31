// https://github.com/inejge/ldap3

use ldap3::result::Result;
use ldap3::{LdapConn, LdapResult};

pub async fn ldap_bind(
    ldap_ip: String,
    ldap_port: String,
    ldap_bind: &str,
    ldap_secret: &str,
) -> Result<LdapResult> {
    ldap_bind_blocking(ldap_ip, ldap_port, ldap_bind, ldap_secret)
}

pub fn ldap_bind_blocking(
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
    ldap.unbind()
}
