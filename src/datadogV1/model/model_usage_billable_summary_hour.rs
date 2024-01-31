// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with monthly summary of data billed by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryHour {
    /// The billing plan.
    #[serde(rename = "billing_plan")]
    pub billing_plan: Option<String>,
    /// Shows the last date of usage.
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// The number of organizations.
    #[serde(rename = "num_orgs")]
    pub num_orgs: Option<i64>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Shows usage aggregation for a billing period.
    #[serde(rename = "ratio_in_month")]
    pub ratio_in_month: Option<f64>,
    /// The region of the organization.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Shows the first date of usage.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// Response with aggregated usage types.
    #[serde(rename = "usage")]
    pub usage: Option<crate::datadogV1::model::UsageBillableSummaryKeys>,
}

impl UsageBillableSummaryHour {
    pub fn new() -> UsageBillableSummaryHour {
        UsageBillableSummaryHour {
            billing_plan: None,
            end_date: None,
            num_orgs: None,
            org_name: None,
            public_id: None,
            ratio_in_month: None,
            region: None,
            start_date: None,
            usage: None,
        }
    }

    pub fn with_billing_plan(&mut self, value: String) -> &mut Self {
        self.billing_plan = Some(value);
        self
    }

    pub fn with_end_date(&mut self, value: String) -> &mut Self {
        self.end_date = Some(value);
        self
    }

    pub fn with_num_orgs(&mut self, value: i64) -> &mut Self {
        self.num_orgs = Some(value);
        self
    }

    pub fn with_org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn with_public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn with_ratio_in_month(&mut self, value: f64) -> &mut Self {
        self.ratio_in_month = Some(value);
        self
    }

    pub fn with_region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn with_start_date(&mut self, value: String) -> &mut Self {
        self.start_date = Some(value);
        self
    }

    pub fn with_usage(
        &mut self,
        value: crate::datadogV1::model::UsageBillableSummaryKeys,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for UsageBillableSummaryHour {
    fn default() -> Self {
        Self::new()
    }
}
