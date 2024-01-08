// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving all monitor configuration policies.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyListResponse {
    /// An array of monitor configuration policies.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::MonitorConfigPolicyResponseData>>,
}

impl MonitorConfigPolicyListResponse {
    pub fn new() -> MonitorConfigPolicyListResponse {
        MonitorConfigPolicyListResponse { data: None }
    }
}
impl Default for MonitorConfigPolicyListResponse {
    fn default() -> Self {
        Self::new()
    }
}
