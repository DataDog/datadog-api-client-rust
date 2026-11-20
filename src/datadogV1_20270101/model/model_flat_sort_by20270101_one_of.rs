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
pub struct FlatSortBy20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1_20270101::model::HeatGridSortAggregation,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1_20270101::model::QuerySortOrder,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "property")]
    pub property: crate::datadogV1_20270101::model::FlatSortByAnyOfAnyOfItemProperty,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FlatSortBy20270101OneOf {
    pub fn new(aggregation: crate::datadogV1_20270101::model::HeatGridSortAggregation, order: crate::datadogV1_20270101::model::QuerySortOrder, property: crate::datadogV1_20270101::model::FlatSortByAnyOfAnyOfItemProperty) -> FlatSortBy20270101OneOf {
        FlatSortBy20270101OneOf {
            aggregation,
            order,
            property,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for FlatSortBy20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlatSortBy20270101OneOfVisitor;
        impl<'a> Visitor<'a> for FlatSortBy20270101OneOfVisitor {
            type Value = FlatSortBy20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1_20270101::model::HeatGridSortAggregation> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut property: Option<crate::datadogV1_20270101::model::FlatSortByAnyOfAnyOfItemProperty> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1_20270101::model::HeatGridSortAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _property) = property {
                                match _property {
                                    crate::datadogV1_20270101::model::FlatSortByAnyOfAnyOfItemProperty::UnparsedObject(_property) => {
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
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = FlatSortBy20270101OneOf {
                    aggregation,
                    order,
                    property,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlatSortBy20270101OneOfVisitor)
    }
}
