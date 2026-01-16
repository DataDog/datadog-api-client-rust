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
pub struct StatusPageDataAttributesComponentsItems {
    /// If the component is of type `group`, the components within the group.
    #[serde(rename = "components")]
    pub components: Option<
        Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItemsComponentsItems>,
    >,
    /// The ID of the component.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// The name of the component.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The zero-indexed position of the component.
    #[serde(rename = "position")]
    pub position: Option<i64>,
    /// The status of the component.
    #[serde(rename = "status")]
    pub status:
        Option<crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsStatus>,
    /// The type of the component.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CreateComponentRequestDataAttributesType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageDataAttributesComponentsItems {
    pub fn new() -> StatusPageDataAttributesComponentsItems {
        StatusPageDataAttributesComponentsItems {
            components: None,
            id: None,
            name: None,
            position: None,
            status: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components(
        mut self,
        value: Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItemsComponentsItems>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::CreateComponentRequestDataAttributesType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for StatusPageDataAttributesComponentsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StatusPageDataAttributesComponentsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageDataAttributesComponentsItemsVisitor;
        impl<'a> Visitor<'a> for StatusPageDataAttributesComponentsItemsVisitor {
            type Value = StatusPageDataAttributesComponentsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components: Option<Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItemsComponentsItems>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut name: Option<String> = None;
                let mut position: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsStatus> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CreateComponentRequestDataAttributesType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "position" => {
                            if v.is_null() {
                                continue;
                            }
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::StatusPagesComponentGroupAttributesComponentsItemsStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CreateComponentRequestDataAttributesType::UnparsedObject(_type_) => {
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

                let content = StatusPageDataAttributesComponentsItems {
                    components,
                    id,
                    name,
                    position,
                    status,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageDataAttributesComponentsItemsVisitor)
    }
}
