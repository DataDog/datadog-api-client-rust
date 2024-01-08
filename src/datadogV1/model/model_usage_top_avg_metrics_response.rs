// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of hourly recorded custom metrics for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::UsageTopAvgMetricsMetadata>>,
    /// Number of hourly recorded custom metrics for a given organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageTopAvgMetricsHour>>,
}

impl UsageTopAvgMetricsResponse {
    pub fn new() -> UsageTopAvgMetricsResponse {
        UsageTopAvgMetricsResponse {
            metadata: None,
            usage: None,
        }
    }
}
impl Default for UsageTopAvgMetricsResponse {
    fn default() -> Self {
        Self::new()
    }
}
