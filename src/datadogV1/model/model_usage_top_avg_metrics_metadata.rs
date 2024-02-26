// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing document metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsMetadata {
    /// The day value from the user request that contains the returned usage data. (If day was used the request)
    #[serde(rename = "day")]
    pub day: Option<String>,
    /// The month value from the user request that contains the returned usage data. (If month was used the request)
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// The metadata for the current pagination.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV1::model::UsageTopAvgMetricsPagination>,
}

impl UsageTopAvgMetricsMetadata {
    pub fn new() -> UsageTopAvgMetricsMetadata {
        UsageTopAvgMetricsMetadata {
            day: None,
            month: None,
            pagination: None,
        }
    }

    pub fn day(&mut self, value: String) -> &mut Self {
        self.day = Some(value);
        self
    }

    pub fn month(&mut self, value: String) -> &mut Self {
        self.month = Some(value);
        self
    }

    pub fn pagination(
        &mut self,
        value: crate::datadogV1::model::UsageTopAvgMetricsPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for UsageTopAvgMetricsMetadata {
    fn default() -> Self {
        Self::new()
    }
}
