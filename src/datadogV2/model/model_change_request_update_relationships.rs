// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for updating a change request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestUpdateRelationships {
    /// Relationship to change request decisions.
    #[serde(rename = "change_request_decisions")]
    pub change_request_decisions:
        Option<crate::datadogV2::model::ChangeRequestDecisionsRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestUpdateRelationships {
    pub fn new() -> ChangeRequestUpdateRelationships {
        ChangeRequestUpdateRelationships {
            change_request_decisions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_request_decisions(
        mut self,
        value: crate::datadogV2::model::ChangeRequestDecisionsRelationship,
    ) -> Self {
        self.change_request_decisions = Some(value);
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

impl Default for ChangeRequestUpdateRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeRequestUpdateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestUpdateRelationshipsVisitor;
        impl<'a> Visitor<'a> for ChangeRequestUpdateRelationshipsVisitor {
            type Value = ChangeRequestUpdateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_request_decisions: Option<
                    crate::datadogV2::model::ChangeRequestDecisionsRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_request_decisions" => {
                            if v.is_null() {
                                continue;
                            }
                            change_request_decisions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChangeRequestUpdateRelationships {
                    change_request_decisions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestUpdateRelationshipsVisitor)
    }
}
