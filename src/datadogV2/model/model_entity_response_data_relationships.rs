// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Entity relationships including incidents, oncalls, schemas, and related entities.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityResponseDataRelationships {
    /// Incidents relationship containing a list of incident resources associated with this entity.
    #[serde(rename = "incidents")]
    pub incidents: Option<crate::datadogV2::model::EntityResponseDataRelationshipsIncidents>,
    /// Oncalls relationship containing a list of oncall resources associated with this entity.
    #[serde(rename = "oncalls")]
    pub oncalls: Option<crate::datadogV2::model::EntityResponseDataRelationshipsOncalls>,
    /// Raw schema relationship linking an entity to its raw schema resource.
    #[serde(rename = "rawSchema")]
    pub raw_schema: Option<crate::datadogV2::model::EntityResponseDataRelationshipsRawSchema>,
    /// Related entities relationship containing a list of entity references related to this entity.
    #[serde(rename = "relatedEntities")]
    pub related_entities:
        Option<crate::datadogV2::model::EntityResponseDataRelationshipsRelatedEntities>,
    /// Schema relationship linking an entity to its associated schema resource.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::EntityResponseDataRelationshipsSchema>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityResponseDataRelationships {
    pub fn new() -> EntityResponseDataRelationships {
        EntityResponseDataRelationships {
            incidents: None,
            oncalls: None,
            raw_schema: None,
            related_entities: None,
            schema: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn incidents(
        mut self,
        value: crate::datadogV2::model::EntityResponseDataRelationshipsIncidents,
    ) -> Self {
        self.incidents = Some(value);
        self
    }

    pub fn oncalls(
        mut self,
        value: crate::datadogV2::model::EntityResponseDataRelationshipsOncalls,
    ) -> Self {
        self.oncalls = Some(value);
        self
    }

    pub fn raw_schema(
        mut self,
        value: crate::datadogV2::model::EntityResponseDataRelationshipsRawSchema,
    ) -> Self {
        self.raw_schema = Some(value);
        self
    }

    pub fn related_entities(
        mut self,
        value: crate::datadogV2::model::EntityResponseDataRelationshipsRelatedEntities,
    ) -> Self {
        self.related_entities = Some(value);
        self
    }

    pub fn schema(
        mut self,
        value: crate::datadogV2::model::EntityResponseDataRelationshipsSchema,
    ) -> Self {
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

impl Default for EntityResponseDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityResponseDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityResponseDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for EntityResponseDataRelationshipsVisitor {
            type Value = EntityResponseDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incidents: Option<
                    crate::datadogV2::model::EntityResponseDataRelationshipsIncidents,
                > = None;
                let mut oncalls: Option<
                    crate::datadogV2::model::EntityResponseDataRelationshipsOncalls,
                > = None;
                let mut raw_schema: Option<
                    crate::datadogV2::model::EntityResponseDataRelationshipsRawSchema,
                > = None;
                let mut related_entities: Option<
                    crate::datadogV2::model::EntityResponseDataRelationshipsRelatedEntities,
                > = None;
                let mut schema: Option<
                    crate::datadogV2::model::EntityResponseDataRelationshipsSchema,
                > = None;
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
                        "oncalls" => {
                            if v.is_null() {
                                continue;
                            }
                            oncalls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = EntityResponseDataRelationships {
                    incidents,
                    oncalls,
                    raw_schema,
                    related_entities,
                    schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityResponseDataRelationshipsVisitor)
    }
}
