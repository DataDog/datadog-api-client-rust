// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with properties for each aggregated usage type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryBody {
    /// The total account usage.
    #[serde(rename = "account_billable_usage")]
    pub account_billable_usage: Option<i64>,
    /// Elapsed usage hours for some billable product.
    #[serde(rename = "elapsed_usage_hours")]
    pub elapsed_usage_hours: Option<i64>,
    /// The first billable hour for the org.
    #[serde(rename = "first_billable_usage_hour")]
    pub first_billable_usage_hour: Option<String>,
    /// The last billable hour for the org.
    #[serde(rename = "last_billable_usage_hour")]
    pub last_billable_usage_hour: Option<String>,
    /// The number of units used within the billable timeframe.
    #[serde(rename = "org_billable_usage")]
    pub org_billable_usage: Option<i64>,
    /// The percentage of account usage the org represents.
    #[serde(rename = "percentage_in_account")]
    pub percentage_in_account: Option<f64>,
    /// Units pertaining to the usage.
    #[serde(rename = "usage_unit")]
    pub usage_unit: Option<String>,
}

impl UsageBillableSummaryBody {
    pub fn new() -> UsageBillableSummaryBody {
        UsageBillableSummaryBody {
            account_billable_usage: None,
            elapsed_usage_hours: None,
            first_billable_usage_hour: None,
            last_billable_usage_hour: None,
            org_billable_usage: None,
            percentage_in_account: None,
            usage_unit: None,
        }
    }

    pub fn account_billable_usage(&mut self, value: i64) -> &mut Self {
        self.account_billable_usage = Some(value);
        self
    }

    pub fn elapsed_usage_hours(&mut self, value: i64) -> &mut Self {
        self.elapsed_usage_hours = Some(value);
        self
    }

    pub fn first_billable_usage_hour(&mut self, value: String) -> &mut Self {
        self.first_billable_usage_hour = Some(value);
        self
    }

    pub fn last_billable_usage_hour(&mut self, value: String) -> &mut Self {
        self.last_billable_usage_hour = Some(value);
        self
    }

    pub fn org_billable_usage(&mut self, value: i64) -> &mut Self {
        self.org_billable_usage = Some(value);
        self
    }

    pub fn percentage_in_account(&mut self, value: f64) -> &mut Self {
        self.percentage_in_account = Some(value);
        self
    }

    pub fn usage_unit(&mut self, value: String) -> &mut Self {
        self.usage_unit = Some(value);
        self
    }
}

impl Default for UsageBillableSummaryBody {
    fn default() -> Self {
        Self::new()
    }
}
