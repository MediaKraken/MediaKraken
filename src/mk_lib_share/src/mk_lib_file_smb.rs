//! Module providing SMB-related functionality for file sharing
//!
//! This module re-exports SMB-related functions and types from the
//! underlying implementations in the library.

pub use crate::mk_lib_smb_pavao::*;
pub use crate::mk_lib_smb_smbclient::{classify_smbclient_browse_error, is_smb_ls_date};
