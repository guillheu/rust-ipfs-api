// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::{Deserialize};



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinRemoteAddResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub pins: Vec<String>,
    pub progress: Option<i32>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinRemoteServiceLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub remote_services: Vec<RemoteService>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteService {
    pub api_endpoint: String,
    pub service: String,
    pub stat: Option<RemoteServiceStat>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteServiceStat {
    pub pin_count: RemoteServiceStatPinCount,
    pub status: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteServiceStatPinCount {
    pub failed: i32,
    pub pinned: i32,
    pub pinning: i32,
    pub queued: i32,
}
