// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing information about multiple roles.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RolesResponse {
    /// Array of returned roles.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::Role>>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::ResponseMetaAttributes>>,
}

impl RolesResponse {
    pub fn new() -> RolesResponse {
        RolesResponse {
            data: None,
            meta: None,
        }
    }
}