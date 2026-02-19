// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a change request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestRelationships {
    /// Relationship to change request decisions.
    #[serde(rename = "change_request_decisions")]
    pub change_request_decisions: crate::datadogV2::model::ChangeRequestDecisionsRelationship,
    /// Relationship to a user.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::ChangeRequestUserRelationship,
    /// Relationship to a user.
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::ChangeRequestUserRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestRelationships {
    pub fn new(
        change_request_decisions: crate::datadogV2::model::ChangeRequestDecisionsRelationship,
        created_by: crate::datadogV2::model::ChangeRequestUserRelationship,
        modified_by: crate::datadogV2::model::ChangeRequestUserRelationship,
    ) -> ChangeRequestRelationships {
        ChangeRequestRelationships {
            change_request_decisions,
            created_by,
            modified_by,
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

impl<'de> Deserialize<'de> for ChangeRequestRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestRelationshipsVisitor;
        impl<'a> Visitor<'a> for ChangeRequestRelationshipsVisitor {
            type Value = ChangeRequestRelationships;

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
                let mut created_by: Option<crate::datadogV2::model::ChangeRequestUserRelationship> =
                    None;
                let mut modified_by: Option<
                    crate::datadogV2::model::ChangeRequestUserRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_request_decisions" => {
                            change_request_decisions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let change_request_decisions = change_request_decisions
                    .ok_or_else(|| M::Error::missing_field("change_request_decisions"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;

                let content = ChangeRequestRelationships {
                    change_request_decisions,
                    created_by,
                    modified_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestRelationshipsVisitor)
    }
}
