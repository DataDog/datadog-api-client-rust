// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountResponse {
    /// Data object of a Cloudflare account.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::datadogV2::model::CloudflareAccountResponseData>>,
}

impl CloudflareAccountResponse {
    /// The expected response schema when getting a Cloudflare account.
    pub fn new() -> CloudflareAccountResponse {
        CloudflareAccountResponse { data: None }
    }
}
