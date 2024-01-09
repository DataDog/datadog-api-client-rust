// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A response list of all service level objective deleted.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLODeleteResponse {
    /// An array containing the ID of the deleted service level objective object.
    #[serde(rename = "data")]
    pub data: Option<Vec<String>>,
    /// An dictionary containing the ID of the SLO as key and a deletion error as value.
    #[serde(rename = "errors")]
    pub errors: Option<std::collections::BTreeMap<String, String>>,
}

impl SLODeleteResponse {
    pub fn new() -> SLODeleteResponse {
        SLODeleteResponse {
            data: None,
            errors: None,
        }
    }
}
impl Default for SLODeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}
