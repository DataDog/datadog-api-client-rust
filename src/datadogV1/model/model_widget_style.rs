// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Widget style definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetStyle {
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetStyle {
    pub fn new() -> WidgetStyle {
        WidgetStyle {
            palette: None,
            _unparsed: false,
        }
    }

    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
}

impl Default for WidgetStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetStyleVisitor;
        impl<'a> Visitor<'a> for WidgetStyleVisitor {
            type Value = WidgetStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WidgetStyle { palette, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetStyleVisitor)
    }
}
