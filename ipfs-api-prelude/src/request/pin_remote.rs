// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;


#[derive(Serialize)]
pub struct PinRemoteLs<'a> {
    pub service: Option<&'a str>,
    pub name: Option<&'a str>,
    pub cid: Option<Vec<&'a str>>,
    pub status: Option<Vec<&'a str>>,
}

impl<'a> ApiRequest for PinRemoteLs<'a> {
    const PATH: &'static str = "/pin/remote/ls";
}