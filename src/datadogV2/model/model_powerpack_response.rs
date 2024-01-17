// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object which includes a single powerpack configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackResponse {
    /// Powerpack data object.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::PowerpackData>>,
    /// Array of objects related to the users.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::User>>,
}

impl PowerpackResponse {
    pub fn new() -> PowerpackResponse {
        PowerpackResponse {
            data: None,
            included: None,
        }
    }
}
impl Default for PowerpackResponse {
    fn default() -> Self {
        Self::new()
    }
}
