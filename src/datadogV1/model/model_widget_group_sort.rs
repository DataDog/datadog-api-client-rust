// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The group to sort the widget by.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetGroupSort {
    /// The name of the group.
    #[serde(rename = "name")]
    pub name: String,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
    /// Set the sort type to group.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::GroupType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetGroupSort {
    pub fn new(
        name: String,
        order: crate::datadogV1::model::WidgetSort,
        type_: crate::datadogV1::model::GroupType,
    ) -> WidgetGroupSort {
        WidgetGroupSort {
            name,
            order,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for WidgetGroupSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetGroupSortVisitor;
        impl<'a> Visitor<'a> for WidgetGroupSortVisitor {
            type Value = WidgetGroupSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
                let mut type_: Option<crate::datadogV1::model::GroupType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::GroupType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = WidgetGroupSort {
                    name,
                    order,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetGroupSortVisitor)
    }
}
