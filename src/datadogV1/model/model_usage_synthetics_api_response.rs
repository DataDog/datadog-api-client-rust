// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of Synthetics API tests run for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSyntheticsAPIResponse {
    /// Get hourly usage for Synthetics API tests.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageSyntheticsAPIHour>>,
}

impl UsageSyntheticsAPIResponse {
    pub fn new() -> UsageSyntheticsAPIResponse {
        UsageSyntheticsAPIResponse { usage: None }
    }
}
impl Default for UsageSyntheticsAPIResponse {
    fn default() -> Self {
        Self::new()
    }
}
