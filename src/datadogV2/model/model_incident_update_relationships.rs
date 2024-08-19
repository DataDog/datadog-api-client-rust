// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's relationships for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentUpdateRelationships {
    /// Relationship to user.
    #[serde(
        rename = "commander_user",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub commander_user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas>,
    /// A relationship reference for postmortems.
    #[serde(rename = "postmortem")]
    pub postmortem: Option<crate::datadogV2::model::RelationshipToIncidentPostmortem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentUpdateRelationships {
    pub fn new() -> IncidentUpdateRelationships {
        IncidentUpdateRelationships {
            commander_user: None,
            integrations: None,
            postmortem: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn commander_user(
        mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> Self {
        self.commander_user = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
    ) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn postmortem(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentPostmortem,
    ) -> Self {
        self.postmortem = Some(value);
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

impl Default for IncidentUpdateRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentUpdateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentUpdateRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentUpdateRelationshipsVisitor {
            type Value = IncidentUpdateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commander_user: Option<
                    Option<crate::datadogV2::model::NullableRelationshipToUser>,
                > = None;
                let mut integrations: Option<
                    crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
                > = None;
                let mut postmortem: Option<
                    crate::datadogV2::model::RelationshipToIncidentPostmortem,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commander_user" => {
                            commander_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "postmortem" => {
                            if v.is_null() {
                                continue;
                            }
                            postmortem = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentUpdateRelationships {
                    commander_user,
                    integrations,
                    postmortem,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentUpdateRelationshipsVisitor)
    }
}
