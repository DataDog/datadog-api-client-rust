// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for creating a change request decision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestDecisionCreateRelationships {
    /// Relationship to a user.
    #[serde(rename = "requested_user")]
    pub requested_user: Option<crate::datadogV2::model::ChangeRequestUserRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestDecisionCreateRelationships {
    pub fn new() -> ChangeRequestDecisionCreateRelationships {
        ChangeRequestDecisionCreateRelationships {
            requested_user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn requested_user(
        mut self,
        value: crate::datadogV2::model::ChangeRequestUserRelationship,
    ) -> Self {
        self.requested_user = Some(value);
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

impl Default for ChangeRequestDecisionCreateRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeRequestDecisionCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestDecisionCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for ChangeRequestDecisionCreateRelationshipsVisitor {
            type Value = ChangeRequestDecisionCreateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut requested_user: Option<
                    crate::datadogV2::model::ChangeRequestUserRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "requested_user" => {
                            if v.is_null() {
                                continue;
                            }
                            requested_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChangeRequestDecisionCreateRelationships {
                    requested_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestDecisionCreateRelationshipsVisitor)
    }
}
