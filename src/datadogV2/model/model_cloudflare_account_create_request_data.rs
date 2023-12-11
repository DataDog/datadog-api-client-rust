// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object for creating a Cloudflare account.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudflareAccountCreateRequestData {
    /// Attributes object for creating a Cloudflare account.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::CloudflareAccountCreateRequestAttributes>,
    /// The JSON:API type for this API. Should always be `cloudflare-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudflareAccountType,
}

impl CloudflareAccountCreateRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::CloudflareAccountCreateRequestAttributes,
        type_: crate::datadogV2::model::CloudflareAccountType,
    ) -> CloudflareAccountCreateRequestData {
        CloudflareAccountCreateRequestData {
            attributes: Box::new(attributes),
            type_,
        }
    }
}
