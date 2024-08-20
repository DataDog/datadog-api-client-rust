// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The image widget allows you to embed an image on your dashboard. An image can be a PNG, JPG, or animated GIF. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn has_background(mut self, value: bool) -> Self {
        self.has_background = Some(value);
        self
    }

    pub fn has_border(mut self, value: bool) -> Self {
        self.has_border = Some(value);
        self
    }

    pub fn horizontal_align(
        mut self,
        value: crate::datadogV1::model::WidgetHorizontalAlign,
    ) -> Self {
        self.horizontal_align = Some(value);
        self
    }

    pub fn margin(mut self, value: crate::datadogV1::model::WidgetMargin) -> Self {
        self.margin = Some(value);
        self
    }

    pub fn sizing(mut self, value: crate::datadogV1::model::WidgetImageSizing) -> Self {
        self.sizing = Some(value);
        self
    }

    pub fn url_dark_theme(mut self, value: String) -> Self {
        self.url_dark_theme = Some(value);
        self
    }

    pub fn vertical_align(mut self, value: crate::datadogV1::model::WidgetVerticalAlign) -> Self {
        self.vertical_align = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ImageWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImageWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for ImageWidgetDefinitionVisitor {
            type Value = ImageWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_background: Option<bool> = None;
                let mut has_border: Option<bool> = None;
                let mut horizontal_align: Option<crate::datadogV1::model::WidgetHorizontalAlign> =
                    None;
                let mut margin: Option<crate::datadogV1::model::WidgetMargin> = None;
                let mut sizing: Option<crate::datadogV1::model::WidgetImageSizing> = None;
                let mut type_: Option<crate::datadogV1::model::ImageWidgetDefinitionType> = None;
                let mut url: Option<String> = None;
                let mut url_dark_theme: Option<String> = None;
                let mut vertical_align: Option<crate::datadogV1::model::WidgetVerticalAlign> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_background" => {
                            if v.is_null() {
                                continue;
                            }
                            has_background =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_border" => {
                            if v.is_null() {
                                continue;
                            }
                            has_border = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "horizontal_align" => {
                            if v.is_null() {
                                continue;
                            }
                            horizontal_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _horizontal_align) = horizontal_align {
                                match _horizontal_align {
                                    crate::datadogV1::model::WidgetHorizontalAlign::UnparsedObject(_horizontal_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "margin" => {
                            if v.is_null() {
                                continue;
                            }
                            margin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _margin) = margin {
                                match _margin {
                                    crate::datadogV1::model::WidgetMargin::UnparsedObject(
                                        _margin,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "sizing" => {
                            if v.is_null() {
                                continue;
                            }
                            sizing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sizing) = sizing {
                                match _sizing {
                                    crate::datadogV1::model::WidgetImageSizing::UnparsedObject(
                                        _sizing,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::ImageWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url_dark_theme" => {
                            if v.is_null() {
                                continue;
                            }
                            url_dark_theme =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vertical_align" => {
                            if v.is_null() {
                                continue;
                            }
                            vertical_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _vertical_align) = vertical_align {
                                match _vertical_align {
                                    crate::datadogV1::model::WidgetVerticalAlign::UnparsedObject(_vertical_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = ImageWidgetDefinition {
                    has_background,
                    has_border,
                    horizontal_align,
                    margin,
                    sizing,
                    type_,
                    url,
                    url_dark_theme,
                    vertical_align,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ImageWidgetDefinitionVisitor)
    }
}
