// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// All relationships associated with AuthN Mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingRelationships {
    /// Relationship to role.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV2::model::RelationshipToRole>,
    /// AuthN Mapping relationship to SAML Assertion Attribute.
    #[serde(rename = "saml_assertion_attribute")]
    pub saml_assertion_attribute:
        Option<crate::datadogV2::model::RelationshipToSAMLAssertionAttribute>,
    /// Relationship to team.
    #[serde(rename = "team")]
    pub team: Option<crate::datadogV2::model::RelationshipToTeam>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingRelationships {
    pub fn new() -> AuthNMappingRelationships {
        AuthNMappingRelationships {
            role: None,
            saml_assertion_attribute: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn role(mut self, value: crate::datadogV2::model::RelationshipToRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn saml_assertion_attribute(
        mut self,
        value: crate::datadogV2::model::RelationshipToSAMLAssertionAttribute,
    ) -> Self {
        self.saml_assertion_attribute = Some(value);
        self
    }

    pub fn team(mut self, value: crate::datadogV2::model::RelationshipToTeam) -> Self {
        self.team = Some(value);
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

impl Default for AuthNMappingRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuthNMappingRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingRelationshipsVisitor;
        impl<'a> Visitor<'a> for AuthNMappingRelationshipsVisitor {
            type Value = AuthNMappingRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut role: Option<crate::datadogV2::model::RelationshipToRole> = None;
                let mut saml_assertion_attribute: Option<
                    crate::datadogV2::model::RelationshipToSAMLAssertionAttribute,
                > = None;
                let mut team: Option<crate::datadogV2::model::RelationshipToTeam> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "role" => {
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "saml_assertion_attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            saml_assertion_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AuthNMappingRelationships {
                    role,
                    saml_assertion_attribute,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingRelationshipsVisitor)
    }
}
