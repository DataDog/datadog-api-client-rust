// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relation relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationRelationships {
    /// Relation to entity.
    #[serde(rename = "fromEntity")]
    pub from_entity: Option<crate::datadogV2::model::RelationToEntity>,
    /// Relation to entity.
    #[serde(rename = "toEntity")]
    pub to_entity: Option<crate::datadogV2::model::RelationToEntity>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RelationRelationships {
    pub fn new() -> RelationRelationships {
        RelationRelationships {
            from_entity: None,
            to_entity: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from_entity(mut self, value: crate::datadogV2::model::RelationToEntity) -> Self {
        self.from_entity = Some(value);
        self
    }

    pub fn to_entity(mut self, value: crate::datadogV2::model::RelationToEntity) -> Self {
        self.to_entity = Some(value);
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

impl Default for RelationRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RelationRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RelationRelationshipsVisitor;
        impl<'a> Visitor<'a> for RelationRelationshipsVisitor {
            type Value = RelationRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_entity: Option<crate::datadogV2::model::RelationToEntity> = None;
                let mut to_entity: Option<crate::datadogV2::model::RelationToEntity> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fromEntity" => {
                            if v.is_null() {
                                continue;
                            }
                            from_entity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "toEntity" => {
                            if v.is_null() {
                                continue;
                            }
                            to_entity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RelationRelationships {
                    from_entity,
                    to_entity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RelationRelationshipsVisitor)
    }
}
