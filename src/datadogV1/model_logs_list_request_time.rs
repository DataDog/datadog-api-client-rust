// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequestTime {
    /// Minimum timestamp for requested logs.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// Timezone can be specified both as an offset (for example "UTC+03:00")
or a regional zone (for example "Europe/Paris").
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
    /// Maximum timestamp for requested logs.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

