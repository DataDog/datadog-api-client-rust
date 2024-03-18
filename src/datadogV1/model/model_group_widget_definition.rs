// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The groups widget allows you to keep similar graphs together on your timeboard. Each group has a custom header, can hold one to many graphs, and is collapsible.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GroupWidgetDefinition {
    /// Background color of the group title.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// URL of image to display as a banner for the group.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// Layout type of the group.
    #[serde(rename = "layout_type")]
    pub layout_type: crate::datadogV1::model::WidgetLayoutType,
    /// Whether to show the title or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Type of the group widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::GroupWidgetDefinitionType,
    /// List of widget groups.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV1::model::Widget>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GroupWidgetDefinition {
    pub fn new(
        layout_type: crate::datadogV1::model::WidgetLayoutType,
        type_: crate::datadogV1::model::GroupWidgetDefinitionType,
        widgets: Vec<crate::datadogV1::model::Widget>,
    ) -> GroupWidgetDefinition {
        GroupWidgetDefinition {
            background_color: None,
            banner_img: None,
            layout_type,
            show_title: None,
            title: None,
            title_align: None,
            type_,
            widgets,
            _unparsed: false,
        }
    }

    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
        self
    }

    pub fn banner_img(mut self, value: String) -> Self {
        self.banner_img = Some(value);
        self
    }

    pub fn show_title(mut self, value: bool) -> Self {
        self.show_title = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for GroupWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GroupWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for GroupWidgetDefinitionVisitor {
            type Value = GroupWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut banner_img: Option<String> = None;
                let mut layout_type: Option<crate::datadogV1::model::WidgetLayoutType> = None;
                let mut show_title: Option<bool> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut type_: Option<crate::datadogV1::model::GroupWidgetDefinitionType> = None;
                let mut widgets: Option<Vec<crate::datadogV1::model::Widget>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "background_color" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner_img" => {
                            if v.is_null() {
                                continue;
                            }
                            banner_img = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "layout_type" => {
                            layout_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout_type) = layout_type {
                                match _layout_type {
                                    crate::datadogV1::model::WidgetLayoutType::UnparsedObject(
                                        _layout_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "show_title" => {
                            if v.is_null() {
                                continue;
                            }
                            show_title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _title_align,
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
                                    crate::datadogV1::model::GroupWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "widgets" => {
                            widgets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let layout_type =
                    layout_type.ok_or_else(|| M::Error::missing_field("layout_type"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let widgets = widgets.ok_or_else(|| M::Error::missing_field("widgets"))?;

                let content = GroupWidgetDefinition {
                    background_color,
                    banner_img,
                    layout_type,
                    show_title,
                    title,
                    title_align,
                    type_,
                    widgets,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GroupWidgetDefinitionVisitor)
    }
}
