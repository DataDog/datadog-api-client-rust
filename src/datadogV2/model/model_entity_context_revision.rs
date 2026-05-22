// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single historical revision of an entity, including the time range during which the revision was observed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityContextRevision {
    /// The set of attributes recorded for the entity at this revision. The keys depend on the kind of entity.
    #[serde(rename = "attributes")]
    pub attributes: std::collections::BTreeMap<String, serde_json::Value>,
    /// The first time the entity was observed at this revision.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    /// The last time the entity was observed at this revision.
    #[serde(rename = "last_seen_at")]
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityContextRevision {
    pub fn new(
        attributes: std::collections::BTreeMap<String, serde_json::Value>,
        first_seen_at: chrono::DateTime<chrono::Utc>,
        last_seen_at: chrono::DateTime<chrono::Utc>,
    ) -> EntityContextRevision {
        EntityContextRevision {
            attributes,
            first_seen_at,
            last_seen_at,
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

impl<'de> Deserialize<'de> for EntityContextRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityContextRevisionVisitor;
        impl<'a> Visitor<'a> for EntityContextRevisionVisitor {
            type Value = EntityContextRevision;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut first_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "first_seen_at" => {
                            first_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen_at" => {
                            last_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let first_seen_at =
                    first_seen_at.ok_or_else(|| M::Error::missing_field("first_seen_at"))?;
                let last_seen_at =
                    last_seen_at.ok_or_else(|| M::Error::missing_field("last_seen_at"))?;

                let content = EntityContextRevision {
                    attributes,
                    first_seen_at,
                    last_seen_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityContextRevisionVisitor)
    }
}
