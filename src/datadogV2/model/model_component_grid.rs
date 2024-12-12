// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ComponentGrid` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ComponentGrid {
    /// The `ComponentGrid` `events`.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV2::model::AppBuilderEvent>>,
    /// The `ComponentGrid` `id`.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The `ComponentGrid` `name`.
    #[serde(rename = "name")]
    pub name: String,
    /// The definition of `ComponentGridProperties` object.
    #[serde(rename = "properties")]
    pub properties: crate::datadogV2::model::ComponentGridProperties,
    /// The definition of `ComponentGridType` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ComponentGridType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ComponentGrid {
    pub fn new(
        name: String,
        properties: crate::datadogV2::model::ComponentGridProperties,
        type_: crate::datadogV2::model::ComponentGridType,
    ) -> ComponentGrid {
        ComponentGrid {
            events: None,
            id: None,
            name,
            properties,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn events(mut self, value: Vec<crate::datadogV2::model::AppBuilderEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for ComponentGrid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ComponentGridVisitor;
        impl<'a> Visitor<'a> for ComponentGridVisitor {
            type Value = ComponentGrid;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut events: Option<Vec<crate::datadogV2::model::AppBuilderEvent>> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut properties: Option<crate::datadogV2::model::ComponentGridProperties> = None;
                let mut type_: Option<crate::datadogV2::model::ComponentGridType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "properties" => {
                            properties = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ComponentGridType::UnparsedObject(
                                        _type_,
                                    ) => {
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
                let properties = properties.ok_or_else(|| M::Error::missing_field("properties"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ComponentGrid {
                    events,
                    id,
                    name,
                    properties,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ComponentGridVisitor)
    }
}
