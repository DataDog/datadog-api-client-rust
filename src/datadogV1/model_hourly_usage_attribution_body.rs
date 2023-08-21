// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageAttributionBody {
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// The name of the organization.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// The source of the usage attribution tag configuration and the selected tags in the format of `<source_org_name>:::<selected tag 1>///<selected tag 2>///<selected tag 3>`.
    #[serde(rename = "tag_config_source", skip_serializing_if = "Option::is_none")]
    pub tag_config_source: String,
    /// Tag keys and values.

A `null` value here means that the requested tag breakdown cannot be applied because it does not match the [tags
configured for usage attribution](https://docs.datadoghq.com/account_management/billing/usage_attribution/#getting-started).
In this scenario the API returns the total usage, not broken down by tags.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tags: map[string]Vec<String>,
    /// Total product usage for the given tags within the hour.
    #[serde(rename = "total_usage_sum", skip_serializing_if = "Option::is_none")]
    pub total_usage_sum: f64,
    /// Shows the most recent hour in the current month for all organizations where usages are calculated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: String,
    /// Supported products for hourly usage attribution requests.
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: HourlyUsageAttributionUsageType,
}

