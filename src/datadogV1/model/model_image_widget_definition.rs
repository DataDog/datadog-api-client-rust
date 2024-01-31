// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The image widget allows you to embed an image on your dashboard. An image can be a PNG, JPG, or animated GIF. Only available on FREE layout dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageWidgetDefinition {
    /// Whether to display a background or not.
    #[serde(rename = "has_background")]
    pub has_background: Option<bool>,
    /// Whether to display a border or not.
    #[serde(rename = "has_border")]
    pub has_border: Option<bool>,
    /// Horizontal alignment.
    #[serde(rename = "horizontal_align")]
    pub horizontal_align: Option<crate::datadogV1::model::WidgetHorizontalAlign>,
    /// Size of the margins around the image.
    /// **Note**: `small` and `large` values are deprecated.
    #[serde(rename = "margin")]
    pub margin: Option<crate::datadogV1::model::WidgetMargin>,
    /// How to size the image on the widget. The values are based on the image `object-fit` CSS properties.
    /// **Note**: `zoom`, `fit` and `center` values are deprecated.
    #[serde(rename = "sizing")]
    pub sizing: Option<crate::datadogV1::model::WidgetImageSizing>,
    /// Type of the image widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ImageWidgetDefinitionType,
    /// URL of the image.
    #[serde(rename = "url")]
    pub url: String,
    /// URL of the image in dark mode.
    #[serde(rename = "url_dark_theme")]
    pub url_dark_theme: Option<String>,
    /// Vertical alignment.
    #[serde(rename = "vertical_align")]
    pub vertical_align: Option<crate::datadogV1::model::WidgetVerticalAlign>,
}

impl ImageWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::ImageWidgetDefinitionType,
        url: String,
    ) -> ImageWidgetDefinition {
        ImageWidgetDefinition {
            has_background: None,
            has_border: None,
            horizontal_align: None,
            margin: None,
            sizing: None,
            type_,
            url,
            url_dark_theme: None,
            vertical_align: None,
        }
    }

    pub fn with_has_background(&mut self, value: bool) -> &mut Self {
        self.has_background = Some(value);
        self
    }

    pub fn with_has_border(&mut self, value: bool) -> &mut Self {
        self.has_border = Some(value);
        self
    }

    pub fn with_horizontal_align(
        &mut self,
        value: crate::datadogV1::model::WidgetHorizontalAlign,
    ) -> &mut Self {
        self.horizontal_align = Some(value);
        self
    }

    pub fn with_margin(&mut self, value: crate::datadogV1::model::WidgetMargin) -> &mut Self {
        self.margin = Some(value);
        self
    }

    pub fn with_sizing(&mut self, value: crate::datadogV1::model::WidgetImageSizing) -> &mut Self {
        self.sizing = Some(value);
        self
    }

    pub fn with_url_dark_theme(&mut self, value: String) -> &mut Self {
        self.url_dark_theme = Some(value);
        self
    }

    pub fn with_vertical_align(
        &mut self,
        value: crate::datadogV1::model::WidgetVerticalAlign,
    ) -> &mut Self {
        self.vertical_align = Some(value);
        self
    }
}
