// https://github.com/pdh11/cotton/tree/main/cotton-ssdp

use serde_json::json;
use stdext::function_name;
use cotton_ssdp::{Advertisement, AsyncService, Notification};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::error::Error;


