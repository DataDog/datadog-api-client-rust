// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the indexed logs usage broken down by retention period for an organization during a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByRetentionResponse {
    /// Get hourly usage for indexed logs by retention period.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageLogsByRetentionHour>>,
}

impl UsageLogsByRetentionResponse {
    pub fn new() -> UsageLogsByRetentionResponse {
        UsageLogsByRetentionResponse { usage: None }
    }
}