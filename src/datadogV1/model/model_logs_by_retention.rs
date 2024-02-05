// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing logs usage data broken down by retention period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetention {
    /// Indexed logs usage summary for each organization for each retention period with usage.
    #[serde(rename = "orgs")]
    pub orgs: Option<crate::datadogV1::model::LogsByRetentionOrgs>,
    /// Aggregated index logs usage for each retention period with usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>>,
    /// Object containing a summary of indexed logs usage by retention period for a single month.
    #[serde(rename = "usage_by_month")]
    pub usage_by_month: Option<crate::datadogV1::model::LogsByRetentionMonthlyUsage>,
}

impl LogsByRetention {
    pub fn new() -> LogsByRetention {
        LogsByRetention {
            orgs: None,
            usage: None,
            usage_by_month: None,
        }
    }

    pub fn orgs(&mut self, value: crate::datadogV1::model::LogsByRetentionOrgs) -> &mut Self {
        self.orgs = Some(value);
        self
    }

    pub fn usage(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsRetentionAggSumUsage>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }

    pub fn usage_by_month(
        &mut self,
        value: crate::datadogV1::model::LogsByRetentionMonthlyUsage,
    ) -> &mut Self {
        self.usage_by_month = Some(value);
        self
    }
}

impl Default for LogsByRetention {
    fn default() -> Self {
        Self::new()
    }
}
