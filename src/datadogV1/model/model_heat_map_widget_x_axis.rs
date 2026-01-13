// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// X Axis controls for the heat map widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatMapWidgetXAxis {
    /// Number of time buckets to target, also known as the resolution
    /// of the time bins. This is only applicable for distribution of
    /// points (group distributions use the roll-up modifier).
    #[serde(rename = "num_buckets")]
    pub num_buckets: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatMapWidgetXAxis {
    pub fn new() -> HeatMapWidgetXAxis {
        HeatMapWidgetXAxis {
            num_buckets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn num_buckets(mut self, value: i64) -> Self {
        self.num_buckets = Some(value);
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

impl Default for HeatMapWidgetXAxis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HeatMapWidgetXAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatMapWidgetXAxisVisitor;
        impl<'a> Visitor<'a> for HeatMapWidgetXAxisVisitor {
            type Value = HeatMapWidgetXAxis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut num_buckets: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "num_buckets" => {
                            if v.is_null() {
                                continue;
                            }
                            num_buckets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HeatMapWidgetXAxis {
                    num_buckets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatMapWidgetXAxisVisitor)
    }
}
