#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/inejge/ldap3
// ldap3 = "0.10.3"

use ldap3::{LdapConnAsync, Scope, SearchEntry};
use ldap3::result::Result;
