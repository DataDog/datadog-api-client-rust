// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a case link.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCaseLinkDataAttributes {
    /// The entity identifier.
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// Whether this is a page link.
    #[serde(rename = "is_page")]
    pub is_page: bool,
    /// The relationship type.
    #[serde(rename = "relationship")]
    pub relationship: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCaseLinkDataAttributes {
    pub fn new(
        entity_id: String,
        is_page: bool,
        relationship: String,
    ) -> IncidentCaseLinkDataAttributes {
        IncidentCaseLinkDataAttributes {
            entity_id,
            is_page,
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

impl<'de> Deserialize<'de> for IncidentCaseLinkDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCaseLinkDataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentCaseLinkDataAttributesVisitor {
            type Value = IncidentCaseLinkDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entity_id: Option<String> = None;
                let mut is_page: Option<bool> = None;
                let mut relationship: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entity_id" => {
                            entity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_page" => {
                            is_page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let entity_id = entity_id.ok_or_else(|| M::Error::missing_field("entity_id"))?;
                let is_page = is_page.ok_or_else(|| M::Error::missing_field("is_page"))?;
                let relationship =
                    relationship.ok_or_else(|| M::Error::missing_field("relationship"))?;

                let content = IncidentCaseLinkDataAttributes {
                    entity_id,
                    is_page,
                    relationship,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCaseLinkDataAttributesVisitor)
    }
}
