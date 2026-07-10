// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The postmortem's relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentPostmortemRelationships {
    /// Relationship to incident.
    #[serde(rename = "incident")]
    pub incident: Option<crate::datadogV2::model::RelationshipToIncident>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// A relationship reference for a single incident responder.
    #[serde(
        rename = "postmortem_owner_responder",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub postmortem_owner_responder:
        Option<Option<crate::datadogV2::model::RelationshipToIncidentResponder>>,
    /// Relationship to user.
    #[serde(
        rename = "postmortem_owner_user",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub postmortem_owner_user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentPostmortemRelationships {
    pub fn new() -> IncidentPostmortemRelationships {
        IncidentPostmortemRelationships {
            incident: None,
            last_modified_by_user: None,
            postmortem_owner_responder: None,
            postmortem_owner_user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn incident(mut self, value: crate::datadogV2::model::RelationshipToIncident) -> Self {
        self.incident = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn postmortem_owner_responder(
        mut self,
        value: Option<crate::datadogV2::model::RelationshipToIncidentResponder>,
    ) -> Self {
        self.postmortem_owner_responder = Some(value);
        self
    }

    pub fn postmortem_owner_user(
        mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> Self {
        self.postmortem_owner_user = Some(value);
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

impl Default for IncidentPostmortemRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentPostmortemRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentPostmortemRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentPostmortemRelationshipsVisitor {
            type Value = IncidentPostmortemRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incident: Option<crate::datadogV2::model::RelationshipToIncident> = None;
                let mut last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut postmortem_owner_responder: Option<
                    Option<crate::datadogV2::model::RelationshipToIncidentResponder>,
                > = None;
                let mut postmortem_owner_user: Option<
                    Option<crate::datadogV2::model::NullableRelationshipToUser>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incident" => {
                            if v.is_null() {
                                continue;
                            }
                            incident = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "postmortem_owner_responder" => {
                            postmortem_owner_responder =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "postmortem_owner_user" => {
                            postmortem_owner_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentPostmortemRelationships {
                    incident,
                    last_modified_by_user,
                    postmortem_owner_responder,
                    postmortem_owner_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentPostmortemRelationshipsVisitor)
    }
}
