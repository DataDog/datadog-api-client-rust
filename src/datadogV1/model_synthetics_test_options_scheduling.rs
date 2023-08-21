// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsScheduling {
    /// Array containing objects describing the scheduling pattern to apply to each day.
    #[serde(rename = "timeframes", skip_serializing_if = "Option::is_none")]
    pub timeframes: Vec<SyntheticsTestOptionsSchedulingTimeframe>,
    /// Timezone in which the timeframe is based.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: String,
}

