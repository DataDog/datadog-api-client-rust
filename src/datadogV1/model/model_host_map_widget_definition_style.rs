// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The style to apply to the widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinitionStyle {
    /// Max value to use to color the map.
    #[serde(rename = "fill_max")]
    pub fill_max: Option<String>,
    /// Min value to use to color the map.
    #[serde(rename = "fill_min")]
    pub fill_min: Option<String>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Whether to flip the palette tones.
    #[serde(rename = "palette_flip")]
    pub palette_flip: Option<bool>,
}

impl HostMapWidgetDefinitionStyle {
    pub fn new() -> HostMapWidgetDefinitionStyle {
        HostMapWidgetDefinitionStyle {
            fill_max: None,
            fill_min: None,
            palette: None,
            palette_flip: None,
        }
    }
}
impl Default for HostMapWidgetDefinitionStyle {
    fn default() -> Self {
        Self::new()
    }
}
