// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountUpdateRequest {
    /// Data object for updating a Cloudflare account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::CloudflareAccountUpdateRequestData>,
}

impl CloudflareAccountUpdateRequest {
    /// Payload schema when updating a Cloudflare account.
    pub fn new(data: crate::datadogV2::model::CloudflareAccountUpdateRequestData) -> CloudflareAccountUpdateRequest {
        CloudflareAccountUpdateRequest { data: Box::new(data) }
    }
}
