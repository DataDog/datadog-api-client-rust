// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's relationships from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentResponseRelationships {
    /// A relationship reference for attachments.
    #[serde(rename = "attachments")]
    pub attachments: Option<crate::datadogV2::model::RelationshipToIncidentAttachment>,
    /// Relationship to user.
    #[serde(
        rename = "commander_user",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub commander_user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to impacts.
    #[serde(rename = "impacts")]
    pub impacts: Option<crate::datadogV2::model::RelationshipToIncidentImpacts>,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to incident responders.
    #[serde(rename = "responders")]
    pub responders: Option<crate::datadogV2::model::RelationshipToIncidentResponders>,
    /// Relationship to incident user defined fields.
    #[serde(rename = "user_defined_fields")]
    pub user_defined_fields:
        Option<crate::datadogV2::model::RelationshipToIncidentUserDefinedFields>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentResponseRelationships {
    pub fn new() -> IncidentResponseRelationships {
        IncidentResponseRelationships {
            attachments: None,
            commander_user: None,
            created_by_user: None,
            impacts: None,
            integrations: None,
            last_modified_by_user: None,
            responders: None,
            user_defined_fields: None,
            _unparsed: false,
        }
    }

    pub fn attachments(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentAttachment,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn commander_user(
        mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> Self {
        self.commander_user = Some(value);
        self
    }

    pub fn created_by_user(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn impacts(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentImpacts,
    ) -> Self {
        self.impacts = Some(value);
        self
    }

    pub fn integrations(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
    ) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn responders(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentResponders,
    ) -> Self {
        self.responders = Some(value);
        self
    }

    pub fn user_defined_fields(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentUserDefinedFields,
    ) -> Self {
        self.user_defined_fields = Some(value);
        self
    }
}

impl Default for IncidentResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentResponseRelationshipsVisitor {
            type Value = IncidentResponseRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attachments: Option<
                    crate::datadogV2::model::RelationshipToIncidentAttachment,
                > = None;
                let mut commander_user: Option<
                    Option<crate::datadogV2::model::NullableRelationshipToUser>,
                > = None;
                let mut created_by_user: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut impacts: Option<crate::datadogV2::model::RelationshipToIncidentImpacts> =
                    None;
                let mut integrations: Option<
                    crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
                > = None;
                let mut last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut responders: Option<
                    crate::datadogV2::model::RelationshipToIncidentResponders,
                > = None;
                let mut user_defined_fields: Option<
                    crate::datadogV2::model::RelationshipToIncidentUserDefinedFields,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attachments" => {
                            if v.is_null() {
                                continue;
                            }
                            attachments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commander_user" => {
                            commander_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impacts" => {
                            if v.is_null() {
                                continue;
                            }
                            impacts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responders" => {
                            if v.is_null() {
                                continue;
                            }
                            responders = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_defined_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            user_defined_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentResponseRelationships {
                    attachments,
                    commander_user,
                    created_by_user,
                    impacts,
                    integrations,
                    last_modified_by_user,
                    responders,
                    user_defined_fields,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentResponseRelationshipsVisitor)
    }
}
