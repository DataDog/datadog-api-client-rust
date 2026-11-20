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
pub struct DatasetSort {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::DatasetSort20270101Aggregation,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column")]
    pub column: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1_20270101::model::QuerySortOrder,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl DatasetSort {
    pub fn new(aggregation: crate::datadogV1_20270101::model::DatasetSort20270101Aggregation, order: crate::datadogV1_20270101::model::QuerySortOrder) -> DatasetSort {
        DatasetSort {
            aggregation,
            column: None,
            order,
            _unparsed: false,
        }
    }
    
    pub fn column(mut self, value: String) -> Self {
        self.column = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for DatasetSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetSortVisitor;
        impl<'a> Visitor<'a> for DatasetSortVisitor {
            type Value = DatasetSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::DatasetSort20270101Aggregation> = None;
                let mut column: Option<String> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1_20270101::model::DatasetSort20270101Aggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_order) => {
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
                let aggregation = aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;

                let content = DatasetSort {
                    aggregation,
                    column,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetSortVisitor)
    }
}
