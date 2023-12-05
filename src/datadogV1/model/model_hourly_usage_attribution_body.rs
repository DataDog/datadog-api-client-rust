// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The usage for one set of tags for one hour.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageAttributionBody {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The name of the organization.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The source of the usage attribution tag configuration and the selected tags in the format of `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source")]
    pub tag_config_source: Option<String>,
    /// Tag keys and values.
    ///
    /// A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
    /// configured for usage attribution](https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started).
    /// In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::HashMap<String, Option<Vec<String>>>>>,
    /// Total product usage for the given tags within the hour.
    #[serde(rename = "total_usage_sum")]
    pub total_usage_sum: Option<f64>,
    /// Shows the most recent hour in the current month for all organizations where usages are calculated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// Supported products for hourly usage attribution requests.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<crate::datadogV1::model::HourlyUsageAttributionUsageType>,
}

impl HourlyUsageAttributionBody {
    pub fn new() -> HourlyUsageAttributionBody {
        HourlyUsageAttributionBody {
            hour: None,
            org_name: None,
            public_id: None,
            region: None,
            tag_config_source: None,
            tags: None,
            total_usage_sum: None,
            updated_at: None,
            usage_type: None,
        }
    }
}