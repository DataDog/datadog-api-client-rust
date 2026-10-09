// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sort rows by their group labels.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridSortByLabel {
    /// Sort direction.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::HeatgridSortOrder,
    /// Sort by label.
    #[serde(rename = "property")]
    pub property: crate::datadogV1::model::HeatgridSortByLabelProperty,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridSortByLabel {
    pub fn new(
        order: crate::datadogV1::model::HeatgridSortOrder,
        property: crate::datadogV1::model::HeatgridSortByLabelProperty,
    ) -> HeatgridSortByLabel {
        HeatgridSortByLabel {
            order,
            property,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridSortByLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridSortByLabelVisitor;
        impl<'a> Visitor<'a> for HeatgridSortByLabelVisitor {
            type Value = HeatgridSortByLabel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut order: Option<crate::datadogV1::model::HeatgridSortOrder> = None;
                let mut property: Option<crate::datadogV1::model::HeatgridSortByLabelProperty> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV1::model::HeatgridSortByLabelProperty::UnparsedObject(_property) => {
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
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = HeatgridSortByLabel {
                    order,
                    property,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridSortByLabelVisitor)
    }
}
