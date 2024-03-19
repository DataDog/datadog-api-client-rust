// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack group widget definition of individual widgets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackInnerWidgets {
    /// Information about widget.
    #[serde(rename = "definition")]
    pub definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// Powerpack inner widget layout.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV2::model::PowerpackInnerWidgetLayout>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackInnerWidgets {
    pub fn new(
        definition: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> PowerpackInnerWidgets {
        PowerpackInnerWidgets {
            definition,
            layout: None,
            _unparsed: false,
        }
    }

    pub fn layout(mut self, value: crate::datadogV2::model::PowerpackInnerWidgetLayout) -> Self {
        self.layout = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackInnerWidgets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackInnerWidgetsVisitor;
        impl<'a> Visitor<'a> for PowerpackInnerWidgetsVisitor {
            type Value = PowerpackInnerWidgets;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut layout: Option<crate::datadogV2::model::PowerpackInnerWidgetLayout> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = PowerpackInnerWidgets {
                    definition,
                    layout,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackInnerWidgetsVisitor)
    }
}
