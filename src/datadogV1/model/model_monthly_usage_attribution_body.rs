// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Usage Summary by tag for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyUsageAttributionBody {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM].
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The source of the usage attribution tag configuration and the selected tags in the format `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    ///
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](<https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started>).
    /// In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, Option<Vec<String>>>>>,
    /// Datetime of the most recent update to the usage values.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Fields in Usage Summary by tag(s).
    #[serde(rename = "values")]
    pub values: Option<crate::datadogV1::model::MonthlyUsageAttributionValues>,
}

impl MonthlyUsageAttributionBody {
    pub fn new() -> MonthlyUsageAttributionBody {
        MonthlyUsageAttributionBody {
            month: None,
            org_name: None,
            public_id: None,
            region: None,
            tag_config_source: None,
            tags: None,
            updated_at: None,
            values: None,
        }
    }

    pub fn with_month(&mut self, value: String) -> &mut Self {
        self.month = Some(value);
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

    pub fn with_region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn with_tag_config_source(&mut self, value: String) -> &mut Self {
        self.tag_config_source = Some(value);
        self
    }

    pub fn with_tags(
        &mut self,
        value: Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
    ) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn with_updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }

    pub fn with_values(
        &mut self,
        value: crate::datadogV1::model::MonthlyUsageAttributionValues,
    ) -> &mut Self {
        self.values = Some(value);
        self
    }
}
impl Default for MonthlyUsageAttributionBody {
    fn default() -> Self {
        Self::new()
    }
}
