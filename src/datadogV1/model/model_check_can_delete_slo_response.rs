// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective response containing the requested object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteSLOResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::CheckCanDeleteSLOResponseData>>,
    /// A mapping of SLO id to it's current usages.
    #[serde(rename = "errors")]
    pub errors: Option<std::collections::BTreeMap<String, String>>,
}

impl CheckCanDeleteSLOResponse {
    pub fn new() -> CheckCanDeleteSLOResponse {
        CheckCanDeleteSLOResponse {
            data: None,
            errors: None,
        }
    }
}
impl Default for CheckCanDeleteSLOResponse {
    fn default() -> Self {
        Self::new()
    }
}
