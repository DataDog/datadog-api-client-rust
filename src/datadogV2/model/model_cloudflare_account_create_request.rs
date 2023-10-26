// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountCreateRequest {
    /// Data object for creating a Cloudflare account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::CloudflareAccountCreateRequestData>,
}

impl CloudflareAccountCreateRequest {
    /// Payload schema when adding a Cloudflare account.
    pub fn new(data: crate::datadogV2::model::CloudflareAccountCreateRequestData) -> CloudflareAccountCreateRequest {
        CloudflareAccountCreateRequest { data: Box::new(data) }
    }
}
