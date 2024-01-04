// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Y Axis controls for the distribution widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetYAxis {
    /// True includes zero.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// The label of the axis to display on the graph.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Specifies the maximum value to show on the y-axis. It takes a number, or auto for default behavior.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Specifies minimum value to show on the y-axis. It takes a number, or auto for default behavior.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear` or `log`.
    #[serde(rename = "scale")]
    pub scale: Option<String>,
}

impl DistributionWidgetYAxis {
    pub fn new() -> DistributionWidgetYAxis {
        DistributionWidgetYAxis {
            include_zero: None,
            label: None,
            max: None,
            min: None,
            scale: None,
        }
    }
}
