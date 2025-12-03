// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Full resource representation of a configured schedule target with position (previous, current, or next).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfiguredSchedule {
    /// Attributes for a configured schedule target, including position.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ConfiguredScheduleTargetAttributes,
    /// Specifies the unique identifier of the configured schedule target.
    #[serde(rename = "id")]
    pub id: String,
    /// Represents the relationships of a configured schedule target.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::ConfiguredScheduleTargetRelationships,
    /// Indicates that the resource is of type `schedule_target`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ConfiguredScheduleTargetType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfiguredSchedule {
    pub fn new(
        attributes: crate::datadogV2::model::ConfiguredScheduleTargetAttributes,
        id: String,
        relationships: crate::datadogV2::model::ConfiguredScheduleTargetRelationships,
        type_: crate::datadogV2::model::ConfiguredScheduleTargetType,
    ) -> ConfiguredSchedule {
        ConfiguredSchedule {
            attributes,
            id,
            relationships,
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

impl<'de> Deserialize<'de> for ConfiguredSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfiguredScheduleVisitor;
        impl<'a> Visitor<'a> for ConfiguredScheduleVisitor {
            type Value = ConfiguredSchedule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::ConfiguredScheduleTargetAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::ConfiguredScheduleTargetRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::ConfiguredScheduleTargetType> = None;
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
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ConfiguredScheduleTargetType::UnparsedObject(_type_) => {
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
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let relationships =
                    relationships.ok_or_else(|| M::Error::missing_field("relationships"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ConfiguredSchedule {
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

        deserializer.deserialize_any(ConfiguredScheduleVisitor)
    }
}
