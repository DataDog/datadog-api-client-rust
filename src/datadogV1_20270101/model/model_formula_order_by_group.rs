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
pub struct FormulaOrderByGroup {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1_20270101::model::QuerySortOrder,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::FormulaOrderByGroupType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaOrderByGroup {
    pub fn new(name: String, order: crate::datadogV1_20270101::model::QuerySortOrder, type_: crate::datadogV1_20270101::model::FormulaOrderByGroupType) -> FormulaOrderByGroup {
        FormulaOrderByGroup {
            name,
            order,
            type_,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for FormulaOrderByGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaOrderByGroupVisitor;
        impl<'a> Visitor<'a> for FormulaOrderByGroupVisitor {
            type Value = FormulaOrderByGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::FormulaOrderByGroupType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::FormulaOrderByGroupType::UnparsedObject(_type_) => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = FormulaOrderByGroup {
                    name,
                    order,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaOrderByGroupVisitor)
    }
}
