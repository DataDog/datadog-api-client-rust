// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Usage Summary by tag for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionBody {
    /// Datetime in ISO-8601 format, UTC, precise to month: [YYYY-MM].
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The source of the usage attribution tag configuration and the selected tags in the format `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    ///
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started).
    /// In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::HashMap<String, Option<Vec<String>>>>>,
    /// Shows the the most recent hour in the current months for all organizations for which all usages were calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Fields in Usage Summary by tag(s).
    #[serde(rename = "values")]
    pub values: Option<Box<crate::datadogV1::model::UsageAttributionValues>>,
}

impl UsageAttributionBody {
    pub fn new() -> UsageAttributionBody {
        UsageAttributionBody {
            month: None,
            org_name: None,
            public_id: None,
            tag_config_source: None,
            tags: None,
            updated_at: None,
            values: None,
        }
    }
}
