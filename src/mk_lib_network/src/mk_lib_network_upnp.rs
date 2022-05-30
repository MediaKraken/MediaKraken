#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/jakobhellermann/rupnp

use futures::prelude::*;
use std::time::Duration;
use rupnp::ssdp::{SearchTarget, URN};
