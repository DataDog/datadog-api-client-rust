// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinitionStyle {
    /// The color palette to apply to the widget.
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: String,
    /// Whether to flip the palette tones.
    #[serde(rename = "palette_flip", skip_serializing_if = "Option::is_none")]
    pub palette_flip: bool,
}

