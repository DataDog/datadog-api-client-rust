// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleOneTimeResponse {
    /// ISO-8601 Datetime to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Time>,
    /// ISO-8601 Datetime to start the downtime.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: String,
}

