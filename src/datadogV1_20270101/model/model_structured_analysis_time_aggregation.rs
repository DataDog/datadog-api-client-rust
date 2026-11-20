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
pub struct StructuredAnalysisTimeAggregation {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_column")]
    pub time_column: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl StructuredAnalysisTimeAggregation {
    pub fn new(time_column: String) -> StructuredAnalysisTimeAggregation {
        StructuredAnalysisTimeAggregation {
            interval: None,
            time_column,
            _unparsed: false,
        }
    }
    
    pub fn interval(mut self, value: f64) -> Self {
        self.interval = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for StructuredAnalysisTimeAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StructuredAnalysisTimeAggregationVisitor;
        impl<'a> Visitor<'a> for StructuredAnalysisTimeAggregationVisitor {
            type Value = StructuredAnalysisTimeAggregation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interval: Option<f64> = None;
                let mut time_column: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interval" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time_column" => {
                            time_column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let time_column = time_column.ok_or_else(|| M::Error::missing_field("time_column"))?;

                let content = StructuredAnalysisTimeAggregation {
                    interval,
                    time_column,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StructuredAnalysisTimeAggregationVisitor)
    }
}
