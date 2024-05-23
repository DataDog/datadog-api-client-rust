// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The formula to sort the widget by.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetFormulaSort {
    /// The index of the formula to sort by.
    #[serde(rename = "index")]
    pub index: i64,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
    /// Set the sort type to formula.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::FormulaType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetFormulaSort {
    pub fn new(
        index: i64,
        order: crate::datadogV1::model::WidgetSort,
        type_: crate::datadogV1::model::FormulaType,
    ) -> WidgetFormulaSort {
        WidgetFormulaSort {
            index,
            order,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for WidgetFormulaSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetFormulaSortVisitor;
        impl<'a> Visitor<'a> for WidgetFormulaSortVisitor {
            type Value = WidgetFormulaSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut index: Option<i64> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
                let mut type_: Option<crate::datadogV1::model::FormulaType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "index" => {
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::WidgetSort::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::FormulaType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let index = index.ok_or_else(|| M::Error::missing_field("index"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = WidgetFormulaSort {
                    index,
                    order,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetFormulaSortVisitor)
    }
}
