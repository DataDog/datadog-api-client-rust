// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByRetentionHour {
    /// Total logs indexed with this retention period during a given hour.
    #[serde(rename = "indexed_events_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub indexed_events_count: Option<Int64>,
    /// Live logs indexed with this retention period during a given hour.
    #[serde(rename = "live_indexed_events_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub live_indexed_events_count: Option<Int64>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// Rehydrated logs indexed with this retention period during a given hour.
    #[serde(rename = "rehydrated_indexed_events_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub rehydrated_indexed_events_count: Option<Int64>,
    /// The retention period in days or "custom" for all custom retention usage.
    #[serde(rename = "retention", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub retention: Option<String>,
}

