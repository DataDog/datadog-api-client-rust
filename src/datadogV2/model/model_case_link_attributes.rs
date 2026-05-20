// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing a directional relationship between two entities (cases, incidents, or pages).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseLinkAttributes {
    /// The UUID of the child (target) entity in the relationship.
    #[serde(rename = "child_entity_id")]
    pub child_entity_id: String,
    /// The type of the child entity. Allowed values: `CASE`, `INCIDENT`, `PAGE`, `AGENT_CONVERSATION`.
    #[serde(rename = "child_entity_type")]
    pub child_entity_type: String,
    /// The UUID of the parent (source) entity in the relationship.
    #[serde(rename = "parent_entity_id")]
    pub parent_entity_id: String,
    /// The type of the parent entity. Allowed values: `CASE`, `INCIDENT`, `PAGE`, `AGENT_CONVERSATION`.
    #[serde(rename = "parent_entity_type")]
    pub parent_entity_type: String,
    /// The type of directional relationship. Allowed values: `RELATES_TO` (bidirectional association), `CAUSES` (parent causes child), `BLOCKS` (parent blocks child), `DUPLICATES` (parent duplicates child), `PARENT_OF` (hierarchical), `SUCCESSOR_OF` (sequence), `ESCALATES_TO` (priority escalation).
    #[serde(rename = "relationship")]
    pub relationship: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseLinkAttributes {
    pub fn new(
        child_entity_id: String,
        child_entity_type: String,
        parent_entity_id: String,
        parent_entity_type: String,
        relationship: String,
    ) -> CaseLinkAttributes {
        CaseLinkAttributes {
            child_entity_id,
            child_entity_type,
            parent_entity_id,
            parent_entity_type,
            relationship,
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

impl<'de> Deserialize<'de> for CaseLinkAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseLinkAttributesVisitor;
        impl<'a> Visitor<'a> for CaseLinkAttributesVisitor {
            type Value = CaseLinkAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child_entity_id: Option<String> = None;
                let mut child_entity_type: Option<String> = None;
                let mut parent_entity_id: Option<String> = None;
                let mut parent_entity_type: Option<String> = None;
                let mut relationship: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "child_entity_id" => {
                            child_entity_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "child_entity_type" => {
                            child_entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_entity_id" => {
                            parent_entity_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_entity_type" => {
                            parent_entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationship" => {
                            relationship =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let child_entity_id =
                    child_entity_id.ok_or_else(|| M::Error::missing_field("child_entity_id"))?;
                let child_entity_type = child_entity_type
                    .ok_or_else(|| M::Error::missing_field("child_entity_type"))?;
                let parent_entity_id =
                    parent_entity_id.ok_or_else(|| M::Error::missing_field("parent_entity_id"))?;
                let parent_entity_type = parent_entity_type
                    .ok_or_else(|| M::Error::missing_field("parent_entity_type"))?;
                let relationship =
                    relationship.ok_or_else(|| M::Error::missing_field("relationship"))?;

                let content = CaseLinkAttributes {
                    child_entity_id,
                    child_entity_type,
                    parent_entity_id,
                    parent_entity_type,
                    relationship,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseLinkAttributesVisitor)
    }
}
