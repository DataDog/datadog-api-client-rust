// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateComponentRequestDataAttributesComponentsItems {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position")]
    pub position: i64,
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateComponentRequestDataAttributesComponentsItems {
    pub fn new(
        name: String,
        position: i64,
        type_: crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsType,
    ) -> CreateComponentRequestDataAttributesComponentsItems {
        CreateComponentRequestDataAttributesComponentsItems {
            name,
            position,
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

impl<'de> Deserialize<'de> for CreateComponentRequestDataAttributesComponentsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateComponentRequestDataAttributesComponentsItemsVisitor;
        impl<'a> Visitor<'a> for CreateComponentRequestDataAttributesComponentsItemsVisitor {
            type Value = CreateComponentRequestDataAttributesComponentsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut position: Option<i64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsType,
                > = None;
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
                        "position" => {
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
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
                let position = position.ok_or_else(|| M::Error::missing_field("position"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CreateComponentRequestDataAttributesComponentsItems {
                    name,
                    position,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateComponentRequestDataAttributesComponentsItemsVisitor)
    }
}
