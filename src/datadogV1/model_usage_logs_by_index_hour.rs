// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsByIndexHour {
    /// The total number of indexed logs for the queried hour.
    #[serde(rename = "event_count", skip_serializing_if = "Option::is_none")]
    pub event_count: i64,
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// The index ID for this usage.
    #[serde(rename = "index_id", skip_serializing_if = "Option::is_none")]
    pub index_id: String,
    /// The user specified name for this index ID.
    #[serde(rename = "index_name", skip_serializing_if = "Option::is_none")]
    pub index_name: String,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The retention period (in days) for this index ID.
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: i64,
}

