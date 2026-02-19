// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a change request decision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestDecisionRelationships {
    /// Relationship to a user.
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::ChangeRequestUserRelationship,
    /// Relationship to a user.
    #[serde(rename = "requested_by_user")]
    pub requested_by_user: crate::datadogV2::model::ChangeRequestUserRelationship,
    /// Relationship to a user.
    #[serde(rename = "requested_user")]
    pub requested_user: crate::datadogV2::model::ChangeRequestUserRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestDecisionRelationships {
    pub fn new(
        modified_by: crate::datadogV2::model::ChangeRequestUserRelationship,
        requested_by_user: crate::datadogV2::model::ChangeRequestUserRelationship,
        requested_user: crate::datadogV2::model::ChangeRequestUserRelationship,
    ) -> ChangeRequestDecisionRelationships {
        ChangeRequestDecisionRelationships {
            modified_by,
            requested_by_user,
            requested_user,
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

impl<'de> Deserialize<'de> for ChangeRequestDecisionRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestDecisionRelationshipsVisitor;
        impl<'a> Visitor<'a> for ChangeRequestDecisionRelationshipsVisitor {
            type Value = ChangeRequestDecisionRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut modified_by: Option<
                    crate::datadogV2::model::ChangeRequestUserRelationship,
                > = None;
                let mut requested_by_user: Option<
                    crate::datadogV2::model::ChangeRequestUserRelationship,
                > = None;
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
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested_by_user" => {
                            requested_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested_user" => {
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
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let requested_by_user = requested_by_user
                    .ok_or_else(|| M::Error::missing_field("requested_by_user"))?;
                let requested_user =
                    requested_user.ok_or_else(|| M::Error::missing_field("requested_user"))?;

                let content = ChangeRequestDecisionRelationships {
                    modified_by,
                    requested_by_user,
                    requested_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestDecisionRelationshipsVisitor)
    }
}
