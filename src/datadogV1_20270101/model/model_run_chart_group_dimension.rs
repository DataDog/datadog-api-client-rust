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
pub struct RunChartGroupDimension {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column")]
    pub column: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1_20270101::model::RunChartGroupDimensionDimension,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RunChartGroupDimension {
    pub fn new(column: String, dimension: crate::datadogV1_20270101::model::RunChartGroupDimensionDimension) -> RunChartGroupDimension {
        RunChartGroupDimension {
            alias: None,
            column,
            dimension,
            _unparsed: false,
        }
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for RunChartGroupDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunChartGroupDimensionVisitor;
        impl<'a> Visitor<'a> for RunChartGroupDimensionVisitor {
            type Value = RunChartGroupDimension;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut column: Option<String> = None;
                let mut dimension: Option<crate::datadogV1_20270101::model::RunChartGroupDimensionDimension> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "dimension" => {
                            dimension = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dimension) = dimension {
                                match _dimension {
                                    crate::datadogV1_20270101::model::RunChartGroupDimensionDimension::UnparsedObject(_dimension) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let dimension = dimension.ok_or_else(|| M::Error::missing_field("dimension"))?;

                let content = RunChartGroupDimension {
                    alias,
                    column,
                    dimension,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RunChartGroupDimensionVisitor)
    }
}
