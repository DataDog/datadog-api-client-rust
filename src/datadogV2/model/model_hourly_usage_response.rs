// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Hourly usage response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageResponse {
    /// Response containing hourly usage.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::HourlyUsage>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::HourlyUsageMetadata>,
}

impl HourlyUsageResponse {
    pub fn new() -> HourlyUsageResponse {
        HourlyUsageResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::HourlyUsage>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::HourlyUsageMetadata) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for HourlyUsageResponse {
    fn default() -> Self {
        Self::new()
    }
}
