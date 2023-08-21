// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookAbsoluteTime {
    /// The end time.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: String,
    /// Indicates whether the timeframe should be shifted to end at the current time.
    #[serde(rename = "live", skip_serializing_if = "Option::is_none")]
    pub live: bool,
    /// The start time.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: String,
}

