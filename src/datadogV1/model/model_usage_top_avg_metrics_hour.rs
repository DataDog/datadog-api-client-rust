// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of hourly recorded custom metrics for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsHour {
    /// Average number of timeseries per hour in which the metric occurs.
    #[serde(rename = "avg_metric_hour")]
    pub avg_metric_hour: Option<i64>,
    /// Maximum number of timeseries per hour in which the metric occurs.
    #[serde(rename = "max_metric_hour")]
    pub max_metric_hour: Option<i64>,
    /// Contains the metric category.
    #[serde(rename = "metric_category")]
    pub metric_category: Option<crate::datadogV1::model::UsageMetricCategory>,
    /// Contains the custom metric name.
    #[serde(rename = "metric_name")]
    pub metric_name: Option<String>,
}

impl UsageTopAvgMetricsHour {
    pub fn new() -> UsageTopAvgMetricsHour {
        UsageTopAvgMetricsHour {
            avg_metric_hour: None,
            max_metric_hour: None,
            metric_category: None,
            metric_name: None,
        }
    }
}
impl Default for UsageTopAvgMetricsHour {
    fn default() -> Self {
        Self::new()
    }
}
