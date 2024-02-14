// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Cost Attribution by Tag for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyCostAttributionAttributes {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]`.
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The source of the cost attribution tag configuration and the selected tags in the format `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](<https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started>).
    /// In this scenario the API returns the total cost, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, Option<Vec<String>>>>>,
    /// Shows the most recent hour in the current months for all organizations for which all costs were calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Fields in Cost Attribution by tag(s). Example: `infra_host_on_demand_cost`, `infra_host_committed_cost`, `infra_host_total_cost`, `infra_host_percentage_in_org`, `infra_host_percentage_in_account`.
    #[serde(rename = "values")]
    pub values: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

impl MonthlyCostAttributionAttributes {
    pub fn new() -> MonthlyCostAttributionAttributes {
        MonthlyCostAttributionAttributes {
            month: None,
            org_name: None,
            public_id: None,
            tag_config_source: None,
            tags: None,
            updated_at: None,
            values: None,
        }
    }

    pub fn month(&mut self, value: String) -> &mut Self {
        self.month = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn tag_config_source(&mut self, value: String) -> &mut Self {
        self.tag_config_source = Some(value);
        self
    }

    pub fn tags(
        &mut self,
        value: Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
    ) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }

    pub fn values(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.values = Some(value);
        self
    }
}

impl Default for MonthlyCostAttributionAttributes {
    fn default() -> Self {
        Self::new()
    }
}
