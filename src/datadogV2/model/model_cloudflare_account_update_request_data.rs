// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object for updating a Cloudflare account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountUpdateRequestData {
    /// Attributes object for updating a Cloudflare account.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::CloudflareAccountUpdateRequestAttributes>>,
    /// The JSON:API type for this API. Should always be `cloudflare-accounts`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CloudflareAccountType>,
}

impl CloudflareAccountUpdateRequestData {
    pub fn new() -> CloudflareAccountUpdateRequestData {
        CloudflareAccountUpdateRequestData {
            attributes: None,
            type_: None,
        }
    }
}
impl Default for CloudflareAccountUpdateRequestData {
    fn default() -> Self {
        Self::new()
    }
}
