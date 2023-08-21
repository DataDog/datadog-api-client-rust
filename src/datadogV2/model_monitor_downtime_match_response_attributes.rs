// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponseAttributes {
    /// The end of the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Time>,
    /// An array of groups associated with the downtime.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Vec<String>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
    /// The start of the downtime.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: String,
}

