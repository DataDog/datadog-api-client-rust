// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeScheduleOneTimeCreateUpdateRequest {
    /// ISO-8601 Datetime to end the downtime. Must include a UTC offset of zero. If not provided, the
downtime continues forever.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Time>,
    /// ISO-8601 Datetime to start the downtime. Must include a UTC offset of zero. If not provided, the
downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub start: Option<Time>,
}

