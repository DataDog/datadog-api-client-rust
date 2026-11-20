// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ImageBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "caption")]
    pub caption: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_background")]
    pub has_background: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_border")]
    pub has_border: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "horizontal_align")]
    pub horizontal_align: Option<crate::datadogV1_20270101::model::ImageBackendDefinitionHorizontalAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "margin")]
    pub margin: Option<crate::datadogV1_20270101::model::ImageMargin>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sizing")]
    pub sizing: Option<crate::datadogV1_20270101::model::ImageBackendDefinition20270101Sizing>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::ImageBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "url")]
    pub url: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "url_dark_theme")]
    pub url_dark_theme: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "vertical_align")]
    pub vertical_align: Option<crate::datadogV1_20270101::model::ImageBackendDefinitionVerticalAlign>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ImageBackendDefinition {
    pub fn new(type_: crate::datadogV1_20270101::model::ImageBackendDefinitionType, url: String) -> ImageBackendDefinition {
        ImageBackendDefinition {
            caption: None,
            has_background: None,
            has_border: None,
            horizontal_align: None,
            margin: None,
            sizing: None,
            type_,
            url,
            url_dark_theme: None,
            vertical_align: None,
            _unparsed: false,
        }
    }
    
    pub fn caption(mut self, value: String) -> Self {
        self.caption = Some(value);
        self
    }
    
    pub fn has_background(mut self, value: bool) -> Self {
        self.has_background = Some(value);
        self
    }
    
    pub fn has_border(mut self, value: bool) -> Self {
        self.has_border = Some(value);
        self
    }
    
    pub fn horizontal_align(mut self, value: crate::datadogV1_20270101::model::ImageBackendDefinitionHorizontalAlign) -> Self {
        self.horizontal_align = Some(value);
        self
    }
    
    pub fn margin(mut self, value: crate::datadogV1_20270101::model::ImageMargin) -> Self {
        self.margin = Some(value);
        self
    }
    
    pub fn sizing(mut self, value: crate::datadogV1_20270101::model::ImageBackendDefinition20270101Sizing) -> Self {
        self.sizing = Some(value);
        self
    }
    
    pub fn url_dark_theme(mut self, value: String) -> Self {
        self.url_dark_theme = Some(value);
        self
    }
    
    pub fn vertical_align(mut self, value: crate::datadogV1_20270101::model::ImageBackendDefinitionVerticalAlign) -> Self {
        self.vertical_align = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ImageBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImageBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for ImageBackendDefinitionVisitor {
            type Value = ImageBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut caption: Option<String> = None;
                let mut has_background: Option<bool> = None;
                let mut has_border: Option<bool> = None;
                let mut horizontal_align: Option<crate::datadogV1_20270101::model::ImageBackendDefinitionHorizontalAlign> = None;
                let mut margin: Option<crate::datadogV1_20270101::model::ImageMargin> = None;
                let mut sizing: Option<crate::datadogV1_20270101::model::ImageBackendDefinition20270101Sizing> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::ImageBackendDefinitionType> = None;
                let mut url: Option<String> = None;
                let mut url_dark_theme: Option<String> = None;
                let mut vertical_align: Option<crate::datadogV1_20270101::model::ImageBackendDefinitionVerticalAlign> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "caption" => {
                            if v.is_null() {
                                continue;
                            }
                            caption = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "has_background" => {
                            if v.is_null() {
                                continue;
                            }
                            has_background = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "has_border" => {
                            if v.is_null() {
                                continue;
                            }
                            has_border = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "horizontal_align" => {
                            if v.is_null() {
                                continue;
                            }
                            horizontal_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _horizontal_align) = horizontal_align {
                                match _horizontal_align {
                                    crate::datadogV1_20270101::model::ImageBackendDefinitionHorizontalAlign::UnparsedObject(_horizontal_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "margin" => {
                            if v.is_null() {
                                continue;
                            }
                            margin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _margin) = margin {
                                match _margin {
                                    crate::datadogV1_20270101::model::ImageMargin::UnparsedObject(_margin) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sizing" => {
                            if v.is_null() {
                                continue;
                            }
                            sizing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sizing) = sizing {
                                match _sizing {
                                    crate::datadogV1_20270101::model::ImageBackendDefinition20270101Sizing::UnparsedObject(_sizing) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::ImageBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "url_dark_theme" => {
                            if v.is_null() {
                                continue;
                            }
                            url_dark_theme = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "vertical_align" => {
                            if v.is_null() {
                                continue;
                            }
                            vertical_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _vertical_align) = vertical_align {
                                match _vertical_align {
                                    crate::datadogV1_20270101::model::ImageBackendDefinitionVerticalAlign::UnparsedObject(_vertical_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = ImageBackendDefinition {
                    caption,
                    has_background,
                    has_border,
                    horizontal_align,
                    margin,
                    sizing,
                    type_,
                    url,
                    url_dark_theme,
                    vertical_align,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ImageBackendDefinitionVisitor)
    }
}
