// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsSchedulingTimeframe {
    /// Number representing the day of the week.
    #[serde(rename = "day", skip_serializing_if = "Option::is_none")]
    pub day: i32,
    /// The hour of the day on which scheduling starts.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// The hour of the day on which scheduling ends.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

