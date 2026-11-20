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
pub struct QueryTableProjection {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column_order")]
    pub column_order: Option<crate::datadogV1_20270101::model::QueryTableProjectionColumnOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dimensions")]
    pub dimensions: Vec<crate::datadogV1_20270101::model::QueryTableProjectionDimension>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::QueryTableProjectionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unlisted_columns")]
    pub unlisted_columns: Option<crate::datadogV1_20270101::model::QueryTableProjectionUnlistedColumns>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableProjection {
    pub fn new(dimensions: Vec<crate::datadogV1_20270101::model::QueryTableProjectionDimension>, type_: crate::datadogV1_20270101::model::QueryTableProjectionType) -> QueryTableProjection {
        QueryTableProjection {
            column_order: None,
            dimensions,
            type_,
            unlisted_columns: None,
            _unparsed: false,
        }
    }
    
    pub fn column_order(mut self, value: crate::datadogV1_20270101::model::QueryTableProjectionColumnOrder) -> Self {
        self.column_order = Some(value);
        self
    }
    
    pub fn unlisted_columns(mut self, value: crate::datadogV1_20270101::model::QueryTableProjectionUnlistedColumns) -> Self {
        self.unlisted_columns = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryTableProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableProjectionVisitor;
        impl<'a> Visitor<'a> for QueryTableProjectionVisitor {
            type Value = QueryTableProjection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_order: Option<crate::datadogV1_20270101::model::QueryTableProjectionColumnOrder> = None;
                let mut dimensions: Option<Vec<crate::datadogV1_20270101::model::QueryTableProjectionDimension>> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::QueryTableProjectionType> = None;
                let mut unlisted_columns: Option<crate::datadogV1_20270101::model::QueryTableProjectionUnlistedColumns> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_order" => {
                            if v.is_null() {
                                continue;
                            }
                            column_order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _column_order) = column_order {
                                match _column_order {
                                    crate::datadogV1_20270101::model::QueryTableProjectionColumnOrder::UnparsedObject(_column_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "dimensions" => {
                            dimensions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::QueryTableProjectionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unlisted_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            unlisted_columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unlisted_columns) = unlisted_columns {
                                match _unlisted_columns {
                                    crate::datadogV1_20270101::model::QueryTableProjectionUnlistedColumns::UnparsedObject(_unlisted_columns) => {
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
                let dimensions = dimensions.ok_or_else(|| M::Error::missing_field("dimensions"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = QueryTableProjection {
                    column_order,
                    dimensions,
                    type_,
                    unlisted_columns,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableProjectionVisitor)
    }
}
