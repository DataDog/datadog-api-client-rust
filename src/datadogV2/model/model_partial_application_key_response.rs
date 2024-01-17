// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving a partial application key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialApplicationKeyResponse {
    /// Partial Datadog application key.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::PartialApplicationKey>>,
    /// Array of objects related to the application key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>>,
}

impl PartialApplicationKeyResponse {
    pub fn new() -> PartialApplicationKeyResponse {
        PartialApplicationKeyResponse {
            data: None,
            included: None,
        }
    }
}
impl Default for PartialApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}
