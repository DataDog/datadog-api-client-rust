// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack group widget object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackGroupWidgetDefinition {
    /// Layout type of widgets.
    #[serde(rename = "layout_type")]
    pub layout_type: String,
    /// Boolean indicating whether powerpack group title should be visible or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Name for the group widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of widget, must be group.
    #[serde(rename = "type")]
    pub type_: String,
    /// Widgets inside the powerpack.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV2::model::PowerpackInnerWidgets>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackGroupWidgetDefinition {
    pub fn new(
        layout_type: String,
        type_: String,
        widgets: Vec<crate::datadogV2::model::PowerpackInnerWidgets>,
    ) -> PowerpackGroupWidgetDefinition {
        PowerpackGroupWidgetDefinition {
            layout_type,
            show_title: None,
            title: None,
            type_,
            widgets,
            _unparsed: false,
        }
    }

    pub fn show_title(mut self, value: bool) -> Self {
        self.show_title = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackGroupWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackGroupWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for PowerpackGroupWidgetDefinitionVisitor {
            type Value = PowerpackGroupWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut layout_type: Option<String> = None;
                let mut show_title: Option<bool> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut widgets: Option<Vec<crate::datadogV2::model::PowerpackInnerWidgets>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "layout_type" => {
                            layout_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = PowerpackGroupWidgetDefinition {
                    layout_type,
                    show_title,
                    title,
                    type_,
                    widgets,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackGroupWidgetDefinitionVisitor)
    }
}
