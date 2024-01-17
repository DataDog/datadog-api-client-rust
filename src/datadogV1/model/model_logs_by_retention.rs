// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing logs usage data broken down by retention period.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetention {
    /// Indexed logs usage summary for each organization for each retention period with usage.
    #[serde(rename = "orgs")]
    pub orgs: Option<Box<crate::datadogV1::model::LogsByRetentionOrgs>>,
    /// Aggregated index logs usage for each retention period with usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>>,
    /// Object containing a summary of indexed logs usage by retention period for a single month.
    #[serde(rename = "usage_by_month")]
    pub usage_by_month: Option<Box<crate::datadogV1::model::LogsByRetentionMonthlyUsage>>,
}

impl LogsByRetention {
    pub fn new() -> LogsByRetention {
        LogsByRetention {
            orgs: None,
            usage: None,
            usage_by_month: None,
        }
    }
}
impl Default for LogsByRetention {
    fn default() -> Self {
        Self::new()
    }
}
