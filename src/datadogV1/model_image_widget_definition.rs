// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageWidgetDefinition {
    /// Whether to display a background or not.
    #[serde(rename = "has_background", skip_serializing_if = "Option::is_none")]
    pub has_background: bool,
    /// Whether to display a border or not.
    #[serde(rename = "has_border", skip_serializing_if = "Option::is_none")]
    pub has_border: bool,
    /// Horizontal alignment.
    #[serde(rename = "horizontal_align", skip_serializing_if = "Option::is_none")]
    pub horizontal_align: WidgetHorizontalAlign,
    /// Size of the margins around the image.
**Note**: `small` and `large` values are deprecated.
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: WidgetMargin,
    /// How to size the image on the widget. The values are based on the image `object-fit` CSS properties.
**Note**: `zoom`, `fit` and `center` values are deprecated.
    #[serde(rename = "sizing", skip_serializing_if = "Option::is_none")]
    pub sizing: WidgetImageSizing,
    /// Type of the image widget.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: ImageWidgetDefinitionType,
    /// URL of the image.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
    /// URL of the image in dark mode.
    #[serde(rename = "url_dark_theme", skip_serializing_if = "Option::is_none")]
    pub url_dark_theme: String,
    /// Vertical alignment.
    #[serde(rename = "vertical_align", skip_serializing_if = "Option::is_none")]
    pub vertical_align: WidgetVerticalAlign,
}

