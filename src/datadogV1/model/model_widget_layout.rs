// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The layout for a widget on a `free` or **new dashboard layout** dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetLayout {
    /// The height of the widget. Should be a non-negative integer.
    #[serde(rename = "height")]
    pub height: i64,
    /// Whether the widget should be the first one on the second column in high density or not.
    /// **Note**: Only for the **new dashboard layout** and only one widget in the dashboard should have this property set to `true`.
    #[serde(rename = "is_column_break")]
    pub is_column_break: Option<bool>,
    /// The width of the widget. Should be a non-negative integer.
    #[serde(rename = "width")]
    pub width: i64,
    /// The position of the widget on the x (horizontal) axis. Should be a non-negative integer.
    #[serde(rename = "x")]
    pub x: i64,
    /// The position of the widget on the y (vertical) axis. Should be a non-negative integer.
    #[serde(rename = "y")]
    pub y: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetLayout {
    pub fn new(height: i64, width: i64, x: i64, y: i64) -> WidgetLayout {
        WidgetLayout {
            height,
            is_column_break: None,
            width,
            x,
            y,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_column_break(mut self, value: bool) -> Self {
        self.is_column_break = Some(value);
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

impl<'de> Deserialize<'de> for WidgetLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetLayoutVisitor;
        impl<'a> Visitor<'a> for WidgetLayoutVisitor {
            type Value = WidgetLayout;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<i64> = None;
                let mut is_column_break: Option<bool> = None;
                let mut width: Option<i64> = None;
                let mut x: Option<i64> = None;
                let mut y: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "height" => {
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_column_break" => {
                            if v.is_null() {
                                continue;
                            }
                            is_column_break =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "x" => {
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let height = height.ok_or_else(|| M::Error::missing_field("height"))?;
                let width = width.ok_or_else(|| M::Error::missing_field("width"))?;
                let x = x.ok_or_else(|| M::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| M::Error::missing_field("y"))?;

                let content = WidgetLayout {
                    height,
                    is_column_break,
                    width,
                    x,
                    y,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetLayoutVisitor)
    }
}
