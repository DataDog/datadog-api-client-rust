// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOThreshold {
    /// The target value for the service level indicator within the corresponding
timeframe.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: f64,
    /// A string representation of the target that indicates its precision.
It uses trailing zeros to show significant decimal places (for example `98.00`).

Always included in service level objective responses. Ignored in
create/update requests.
    #[serde(rename = "target_display", skip_serializing_if = "Option::is_none")]
    pub target_display: String,
    /// The SLO time window options.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: SLOTimeframe,
    /// The warning value for the service level objective.
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: f64,
    /// A string representation of the warning target (see the description of
the `target_display` field for details).

Included in service level objective responses if a warning target exists.
Ignored in create/update requests.
    #[serde(rename = "warning_display", skip_serializing_if = "Option::is_none")]
    pub warning_display: String,
}

