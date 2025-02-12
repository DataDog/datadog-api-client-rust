// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data related to the workflow being updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WorkflowDataUpdate {
    /// The definition of `WorkflowDataUpdateAttributes` object.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::WorkflowDataUpdateAttributes,
    /// The workflow identifier
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The definition of `WorkflowDataRelationships` object.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::WorkflowDataRelationships>,
    /// The definition of `WorkflowDataType` object.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::WorkflowDataType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WorkflowDataUpdate {
    pub fn new(
        attributes: crate::datadogV2::model::WorkflowDataUpdateAttributes,
        type_: crate::datadogV2::model::WorkflowDataType,
    ) -> WorkflowDataUpdate {
        WorkflowDataUpdate {
            attributes,
            id: None,
            relationships: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::WorkflowDataRelationships,
    ) -> Self {
        self.relationships = Some(value);
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

impl<'de> Deserialize<'de> for WorkflowDataUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkflowDataUpdateVisitor;
        impl<'a> Visitor<'a> for WorkflowDataUpdateVisitor {
            type Value = WorkflowDataUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::WorkflowDataUpdateAttributes> =
                    None;
                let mut id: Option<String> = None;
                let mut relationships: Option<crate::datadogV2::model::WorkflowDataRelationships> =
                    None;
                let mut type_: Option<crate::datadogV2::model::WorkflowDataType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            if v.is_null() {
                                continue;
                            }
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::WorkflowDataType::UnparsedObject(
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
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = WorkflowDataUpdate {
                    attributes,
                    id,
                    relationships,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WorkflowDataUpdateVisitor)
    }
}
