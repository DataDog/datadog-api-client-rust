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
pub struct TableTrendCellDisplayModeOptions {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "trend_type")]
    pub trend_type: crate::datadogV1_20270101::model::TableTrendType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "y_scale")]
    pub y_scale: crate::datadogV1_20270101::model::TableTrendCellDisplayModeOptionsYScale,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableTrendCellDisplayModeOptions {
    pub fn new(trend_type: crate::datadogV1_20270101::model::TableTrendType, y_scale: crate::datadogV1_20270101::model::TableTrendCellDisplayModeOptionsYScale) -> TableTrendCellDisplayModeOptions {
        TableTrendCellDisplayModeOptions {
            trend_type,
            y_scale,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for TableTrendCellDisplayModeOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableTrendCellDisplayModeOptionsVisitor;
        impl<'a> Visitor<'a> for TableTrendCellDisplayModeOptionsVisitor {
            type Value = TableTrendCellDisplayModeOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut trend_type: Option<crate::datadogV1_20270101::model::TableTrendType> = None;
                let mut y_scale: Option<crate::datadogV1_20270101::model::TableTrendCellDisplayModeOptionsYScale> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "trend_type" => {
                            trend_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trend_type) = trend_type {
                                match _trend_type {
                                    crate::datadogV1_20270101::model::TableTrendType::UnparsedObject(_trend_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "y_scale" => {
                            y_scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _y_scale) = y_scale {
                                match _y_scale {
                                    crate::datadogV1_20270101::model::TableTrendCellDisplayModeOptionsYScale::UnparsedObject(_y_scale) => {
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
                let trend_type = trend_type.ok_or_else(|| M::Error::missing_field("trend_type"))?;
                let y_scale = y_scale.ok_or_else(|| M::Error::missing_field("y_scale"))?;

                let content = TableTrendCellDisplayModeOptions {
                    trend_type,
                    y_scale,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableTrendCellDisplayModeOptionsVisitor)
    }
}
