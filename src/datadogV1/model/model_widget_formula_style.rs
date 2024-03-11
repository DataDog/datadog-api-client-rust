// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Styling options for widget formulas.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetFormulaStyle {
    /// The color palette used to display the formula. A guide to the available color palettes can be found at <https://docs.datadoghq.com/dashboards/guide/widget_colors>
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Index specifying which color to use within the palette.
    #[serde(rename = "palette_index")]
    pub palette_index: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetFormulaStyle {
    pub fn new() -> WidgetFormulaStyle {
        WidgetFormulaStyle {
            palette: None,
            palette_index: None,
            _unparsed: false,
        }
    }

    pub fn palette(&mut self, value: String) -> &mut Self {
        self.palette = Some(value);
        self
    }

    pub fn palette_index(&mut self, value: i64) -> &mut Self {
        self.palette_index = Some(value);
        self
    }
}

impl Default for WidgetFormulaStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetFormulaStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetFormulaStyleVisitor;
        impl<'a> Visitor<'a> for WidgetFormulaStyleVisitor {
            type Value = WidgetFormulaStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<String> = None;
                let mut palette_index: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette_index" => {
                            if v.is_null() {
                                continue;
                            }
                            palette_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WidgetFormulaStyle {
                    palette,
                    palette_index,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetFormulaStyleVisitor)
    }
}
