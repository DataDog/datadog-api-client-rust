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
pub struct FlatSortBy20270101OneOf4456386848 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::QuerySortOrder,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "property")]
    pub property: crate::datadogV1::model::FlatSortByAnyOfAnyOfItemProperty4446002432,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FlatSortBy20270101OneOf4456386848 {
    pub fn new(order: crate::datadogV1::model::QuerySortOrder, property: crate::datadogV1::model::FlatSortByAnyOfAnyOfItemProperty4446002432) -> FlatSortBy20270101OneOf4456386848 {
        FlatSortBy20270101OneOf4456386848 {
            order,
            property,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for FlatSortBy20270101OneOf4456386848 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlatSortBy20270101OneOf4456386848Visitor;
        impl<'a> Visitor<'a> for FlatSortBy20270101OneOf4456386848Visitor {
            type Value = FlatSortBy20270101OneOf4456386848;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut order: Option<crate::datadogV1::model::QuerySortOrder> = None;
                let mut property: Option<crate::datadogV1::model::FlatSortByAnyOfAnyOfItemProperty4446002432> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::QuerySortOrder::UnparsedObject(_order) => {
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
                                    crate::datadogV1::model::FlatSortByAnyOfAnyOfItemProperty4446002432::UnparsedObject(_property) => {
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
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = FlatSortBy20270101OneOf4456386848 {
                    order,
                    property,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlatSortBy20270101OneOf4456386848Visitor)
    }
}
