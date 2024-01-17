// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Status of downgrade
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgDowngradedResponse {
    /// Information pertaining to the downgraded child organization.
    #[serde(rename = "message")]
    pub message: Option<String>,
}

impl OrgDowngradedResponse {
    pub fn new() -> OrgDowngradedResponse {
        OrgDowngradedResponse { message: None }
    }
}
impl Default for OrgDowngradedResponse {
    fn default() -> Self {
        Self::new()
    }
}
