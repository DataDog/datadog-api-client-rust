// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// SLO thresholds (target and optionally warning) for a single time window.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOThreshold {
    /// The target value for the service level indicator within the corresponding
    /// timeframe.
    #[serde(rename = "target")]
    pub target: f64,
    /// A string representation of the target that indicates its precision.
    /// It uses trailing zeros to show significant decimal places (for example `98.00`).
    ///
    /// Always included in service level objective responses. Ignored in
    /// create/update requests.
    #[serde(rename = "target_display")]
    pub target_display: Option<String>,
    /// The SLO time window options.
    #[serde(rename = "timeframe")]
    pub timeframe: crate::datadogV1::model::SLOTimeframe,
    /// The warning value for the service level objective.
    #[serde(rename = "warning")]
    pub warning: Option<f64>,
    /// A string representation of the warning target (see the description of
    /// the `target_display` field for details).
    ///
    /// Included in service level objective responses if a warning target exists.
    /// Ignored in create/update requests.
    #[serde(rename = "warning_display")]
    pub warning_display: Option<String>,
}

impl SLOThreshold {
    pub fn new(target: f64, timeframe: crate::datadogV1::model::SLOTimeframe) -> SLOThreshold {
        SLOThreshold {
            target,
            target_display: None,
            timeframe,
            warning: None,
            warning_display: None,
        }
    }

    pub fn with_target_display(&mut self, value: String) -> &mut Self {
        self.target_display = Some(value);
        self
    }

    pub fn with_warning(&mut self, value: f64) -> &mut Self {
        self.warning = Some(value);
        self
    }

    pub fn with_warning_display(&mut self, value: String) -> &mut Self {
        self.warning_display = Some(value);
        self
    }
}
