// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorThresholds {
    /// The monitor `CRITICAL` threshold.
    #[serde(rename = "critical", skip_serializing_if = "Option::is_none")]
    pub critical: f64,
    /// The monitor `CRITICAL` recovery threshold.
    #[serde(rename = "critical_recovery", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub critical_recovery: Option<Float64>,
    /// The monitor `OK` threshold.
    #[serde(rename = "ok", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ok: Option<Float64>,
    /// The monitor UNKNOWN threshold.
    #[serde(rename = "unknown", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub unknown: Option<Float64>,
    /// The monitor `WARNING` threshold.
    #[serde(rename = "warning", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub warning: Option<Float64>,
    /// The monitor `WARNING` recovery threshold.
    #[serde(rename = "warning_recovery", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub warning_recovery: Option<Float64>,
}

