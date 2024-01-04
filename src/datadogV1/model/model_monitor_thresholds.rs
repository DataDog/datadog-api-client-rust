// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of the different monitor threshold available.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorThresholds {
    /// The monitor `CRITICAL` threshold.
    #[serde(rename = "critical")]
    pub critical: Option<f64>,
    /// The monitor `CRITICAL` recovery threshold.
    #[serde(
        rename = "critical_recovery",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub critical_recovery: Option<Option<f64>>,
    /// The monitor `OK` threshold.
    #[serde(rename = "ok", default, with = "::serde_with::rust::double_option")]
    pub ok: Option<Option<f64>>,
    /// The monitor UNKNOWN threshold.
    #[serde(
        rename = "unknown",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub unknown: Option<Option<f64>>,
    /// The monitor `WARNING` threshold.
    #[serde(
        rename = "warning",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning: Option<Option<f64>>,
    /// The monitor `WARNING` recovery threshold.
    #[serde(
        rename = "warning_recovery",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning_recovery: Option<Option<f64>>,
}

impl MonitorThresholds {
    pub fn new() -> MonitorThresholds {
        MonitorThresholds {
            critical: None,
            critical_recovery: None,
            ok: None,
            unknown: None,
            warning: None,
            warning_recovery: None,
        }
    }
}
