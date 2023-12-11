// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// SLO thresholds (target and optionally warning) for a single time window.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOThreshold {
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
    pub timeframe: crate::datadogV1::model::SearchSLOTimeframe,
    /// The warning value for the service level objective.
    #[serde(
        rename = "warning",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning: Option<Option<f64>>,
    /// A string representation of the warning target (see the description of
    /// the `target_display` field for details).
    ///
    /// Included in service level objective responses if a warning target exists.
    /// Ignored in create/update requests.
    #[serde(
        rename = "warning_display",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warning_display: Option<Option<String>>,
}

impl SearchSLOThreshold {
    pub fn new(
        target: f64,
        timeframe: crate::datadogV1::model::SearchSLOTimeframe,
    ) -> SearchSLOThreshold {
        SearchSLOThreshold {
            target,
            target_display: None,
            timeframe,
            warning: None,
            warning_display: None,
        }
    }
}
