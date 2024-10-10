// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Oncall escalation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityResponseIncludedRelatedOncallEscalationItem {
    /// Oncall email.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Oncall level.
    #[serde(rename = "escalationLevel")]
    pub escalation_level: Option<i64>,
    /// Oncall name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityResponseIncludedRelatedOncallEscalationItem {
    pub fn new() -> EntityResponseIncludedRelatedOncallEscalationItem {
        EntityResponseIncludedRelatedOncallEscalationItem {
            email: None,
            escalation_level: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn escalation_level(mut self, value: i64) -> Self {
        self.escalation_level = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for EntityResponseIncludedRelatedOncallEscalationItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityResponseIncludedRelatedOncallEscalationItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityResponseIncludedRelatedOncallEscalationItemVisitor;
        impl<'a> Visitor<'a> for EntityResponseIncludedRelatedOncallEscalationItemVisitor {
            type Value = EntityResponseIncludedRelatedOncallEscalationItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<String> = None;
                let mut escalation_level: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "escalationLevel" => {
                            if v.is_null() {
                                continue;
                            }
                            escalation_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EntityResponseIncludedRelatedOncallEscalationItem {
                    email,
                    escalation_level,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityResponseIncludedRelatedOncallEscalationItemVisitor)
    }
}
