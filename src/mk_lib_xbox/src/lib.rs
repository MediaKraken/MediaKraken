//! Rust API for Xbox Live calls.
//!
//! - Auth (user token / XSTS tickets), gamertag→XUID resolution and the
//!   people-search endpoint come from the [`xbox`] crate; they are re-exported
//!   here and available directly on [`XboxApi`].
//! - Any other Xbox Live service endpoint (profile, friends, achievements,
//!   …) can be called with [`XboxApi::call`] / [`XboxApi::call_post`]; the
//!   `XBL3.0 x=<uhs>;<token>` authorization header is built automatically from
//!   a cached XSTS ticket for the requested relying party.

pub mod mk_lib_xbox_api;
pub mod mk_lib_xbox_error;

pub use mk_lib_xbox_api::XboxApi;
pub use mk_lib_xbox_error::{Result, XboxApiError};

pub use xbox::auth::{LegacyPasswordProvider, XblAuthProvider};
pub use xbox::models::{
    DisplayClaims, PeopleSearchResponse, Person, UserToken, XstsToken, XuiItem, Xuid,
};
pub use xbox::{RelyingParty, XboxClient, XboxEndpoints, XboxError, models, util};
