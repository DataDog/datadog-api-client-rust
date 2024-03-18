// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team update request
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamUpdate {
    /// Team update attributes
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::TeamUpdateAttributes,
    /// Team update relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::TeamUpdateRelationships>,
    /// Team type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamUpdate {
    pub fn new(
        attributes: crate::datadogV2::model::TeamUpdateAttributes,
        type_: crate::datadogV2::model::TeamType,
    ) -> TeamUpdate {
        TeamUpdate {
            attributes,
            relationships: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::TeamUpdateRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TeamUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamUpdateVisitor;
        impl<'a> Visitor<'a> for TeamUpdateVisitor {
            type Value = TeamUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::TeamUpdateAttributes> = None;
                let mut relationships: Option<crate::datadogV2::model::TeamUpdateRelationships> =
                    None;
                let mut type_: Option<crate::datadogV2::model::TeamType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::TeamType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TeamUpdate {
                    attributes,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamUpdateVisitor)
    }
}
