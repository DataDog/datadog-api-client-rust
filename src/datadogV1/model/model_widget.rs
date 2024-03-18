// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about widget.
///
/// **Note**: The `layout` property is required for widgets in dashboards with `free` `layout_type`.
///       For the **new dashboard layout**, the `layout` property depends on the `reflow_type` of the dashboard.
///       - If `reflow_type` is `fixed`, `layout` is required.
///       - If `reflow_type` is `auto`, `layout` should not be set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Widget {
    /// [Definition of the widget](<https://docs.datadoghq.com/dashboards/widgets/>).
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::WidgetDefinition,
    /// ID of the widget.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The layout for a widget on a `free` or **new dashboard layout** dashboard.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV1::model::WidgetLayout>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Widget {
    pub fn new(definition: crate::datadogV1::model::WidgetDefinition) -> Widget {
        Widget {
            definition,
            id: None,
            layout: None,
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn layout(mut self, value: crate::datadogV1::model::WidgetLayout) -> Self {
        self.layout = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for Widget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetVisitor;
        impl<'a> Visitor<'a> for WidgetVisitor {
            type Value = Widget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<crate::datadogV1::model::WidgetDefinition> = None;
                let mut id: Option<i64> = None;
                let mut layout: Option<crate::datadogV1::model::WidgetLayout> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _definition) = definition {
                                match _definition {
                                    crate::datadogV1::model::WidgetDefinition::UnparsedObject(
                                        _definition,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "layout" => {
                            if v.is_null() {
                                continue;
                            }
                            layout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;

                let content = Widget {
                    definition,
                    id,
                    layout,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetVisitor)
    }
}
