// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// X Axis controls for the distribution widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetXAxis {
    /// True includes zero.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// Specifies maximum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Specifies minimum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear`.
    #[serde(rename = "scale")]
    pub scale: Option<String>,
}

impl DistributionWidgetXAxis {
    pub fn new() -> DistributionWidgetXAxis {
        DistributionWidgetXAxis {
            include_zero: None,
            max: None,
            min: None,
            scale: None,
        }
    }
}
impl Default for DistributionWidgetXAxis {
    fn default() -> Self {
        Self::new()
    }
}
