// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sort rows by their aggregated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridSortByValue {
    /// Aggregation used to order rows over the displayed time range.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::HeatgridSortAggregation,
    /// Sort direction.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::HeatgridSortOrder,
    /// Sort by value.
    #[serde(rename = "property")]
    pub property: crate::datadogV1::model::HeatgridSortByValueProperty,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridSortByValue {
    pub fn new(
        aggregation: crate::datadogV1::model::HeatgridSortAggregation,
        order: crate::datadogV1::model::HeatgridSortOrder,
        property: crate::datadogV1::model::HeatgridSortByValueProperty,
    ) -> HeatgridSortByValue {
        HeatgridSortByValue {
            aggregation,
            order,
            property,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridSortByValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridSortByValueVisitor;
        impl<'a> Visitor<'a> for HeatgridSortByValueVisitor {
            type Value = HeatgridSortByValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV1::model::HeatgridSortAggregation> =
                    None;
                let mut order: Option<crate::datadogV1::model::HeatgridSortOrder> = None;
                let mut property: Option<crate::datadogV1::model::HeatgridSortByValueProperty> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1::model::HeatgridSortAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::HeatgridSortOrder::UnparsedObject(
                                        _order,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _property) = property {
                                match _property {
                                    crate::datadogV1::model::HeatgridSortByValueProperty::UnparsedObject(_property) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = HeatgridSortByValue {
                    aggregation,
                    order,
                    property,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridSortByValueVisitor)
    }
}
