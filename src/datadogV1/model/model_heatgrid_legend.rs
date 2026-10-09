// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Legend configuration for the heatgrid widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridLegend {
    /// Whether to display the legend caption.
    #[serde(rename = "show_caption")]
    pub show_caption: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridLegend {
    pub fn new() -> HeatgridLegend {
        HeatgridLegend {
            show_caption: None,
            _unparsed: false,
        }
    }

    pub fn show_caption(mut self, value: bool) -> Self {
        self.show_caption = Some(value);
        self
    }
}

impl Default for HeatgridLegend {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HeatgridLegend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridLegendVisitor;
        impl<'a> Visitor<'a> for HeatgridLegendVisitor {
            type Value = HeatgridLegend;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut show_caption: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "show_caption" => {
                            if v.is_null() {
                                continue;
                            }
                            show_caption =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = HeatgridLegend {
                    show_caption,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridLegendVisitor)
    }
}
