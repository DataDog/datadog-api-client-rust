// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The style to apply to the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinitionStyle {
    /// The color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: String,
    /// Whether to flip the palette tones.
    #[serde(rename = "palette_flip")]
    pub palette_flip: bool,
}

impl GeomapWidgetDefinitionStyle {
    pub fn new(palette: String, palette_flip: bool) -> GeomapWidgetDefinitionStyle {
        GeomapWidgetDefinitionStyle {
            palette,
            palette_flip,
        }
    }
}
