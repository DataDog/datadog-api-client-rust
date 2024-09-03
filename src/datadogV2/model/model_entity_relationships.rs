// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Entity relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityRelationships {
    /// Entity to incidents relationship.
    #[serde(rename = "incidents")]
    pub incidents: Option<crate::datadogV2::model::EntityToIncidents>,
    /// Entity to oncalls relationship.
    #[serde(rename = "oncall")]
    pub oncall: Option<crate::datadogV2::model::EntityToOncalls>,
    /// Entity to raw schema relationship.
    #[serde(rename = "rawSchema")]
    pub raw_schema: Option<crate::datadogV2::model::EntityToRawSchema>,
    /// Entity to related entities relationship.
    #[serde(rename = "relatedEntities")]
    pub related_entities: Option<crate::datadogV2::model::EntityToRelatedEntities>,
    /// Entity to detail schema relationship.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::EntityToSchema>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityRelationships {
    pub fn new() -> EntityRelationships {
        EntityRelationships {
            incidents: None,
            oncall: None,
            raw_schema: None,
            related_entities: None,
            schema: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn incidents(mut self, value: crate::datadogV2::model::EntityToIncidents) -> Self {
        self.incidents = Some(value);
        self
    }

    pub fn oncall(mut self, value: crate::datadogV2::model::EntityToOncalls) -> Self {
        self.oncall = Some(value);
        self
    }

    pub fn raw_schema(mut self, value: crate::datadogV2::model::EntityToRawSchema) -> Self {
        self.raw_schema = Some(value);
        self
    }

    pub fn related_entities(
        mut self,
        value: crate::datadogV2::model::EntityToRelatedEntities,
    ) -> Self {
        self.related_entities = Some(value);
        self
    }

    pub fn schema(mut self, value: crate::datadogV2::model::EntityToSchema) -> Self {
        self.schema = Some(value);
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

impl Default for EntityRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityRelationshipsVisitor;
        impl<'a> Visitor<'a> for EntityRelationshipsVisitor {
            type Value = EntityRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incidents: Option<crate::datadogV2::model::EntityToIncidents> = None;
                let mut oncall: Option<crate::datadogV2::model::EntityToOncalls> = None;
                let mut raw_schema: Option<crate::datadogV2::model::EntityToRawSchema> = None;
                let mut related_entities: Option<crate::datadogV2::model::EntityToRelatedEntities> =
                    None;
                let mut schema: Option<crate::datadogV2::model::EntityToSchema> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incidents" => {
                            if v.is_null() {
                                continue;
                            }
                            incidents = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "oncall" => {
                            if v.is_null() {
                                continue;
                            }
                            oncall = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rawSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            raw_schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relatedEntities" => {
                            if v.is_null() {
                                continue;
                            }
                            related_entities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EntityRelationships {
                    incidents,
                    oncall,
                    raw_schema,
                    related_entities,
                    schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityRelationshipsVisitor)
    }
}
