// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of Synthetics Browser tests run for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSyntheticsBrowserResponse {
    /// Get hourly usage for Synthetics Browser tests.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageSyntheticsBrowserHour>>,
}

impl UsageSyntheticsBrowserResponse {
    pub fn new() -> UsageSyntheticsBrowserResponse {
        UsageSyntheticsBrowserResponse { usage: None }
    }
}
impl Default for UsageSyntheticsBrowserResponse {
    fn default() -> Self {
        Self::new()
    }
}
