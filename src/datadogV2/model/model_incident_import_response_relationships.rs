// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's relationships from an import response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImportResponseRelationships {
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
    /// Relationship to user.
    #[serde(rename = "declared_by_user")]
    pub declared_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to impacts.
    #[serde(rename = "impacts")]
    pub impacts: Option<crate::datadogV2::model::RelationshipToIncidentImpacts>,
    /// Relationship to an incident type.
    #[serde(rename = "incident_type")]
    pub incident_type: Option<crate::datadogV2::model::RelationshipToIncidentType>,
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentImportResponseRelationships {
    pub fn new() -> IncidentImportResponseRelationships {
        IncidentImportResponseRelationships {
            attachments: None,
            commander_user: None,
            created_by_user: None,
            declared_by_user: None,
            impacts: None,
            incident_type: None,
            integrations: None,
            last_modified_by_user: None,
            responders: None,
            user_defined_fields: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn declared_by_user(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.declared_by_user = Some(value);
        self
    }

    pub fn impacts(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentImpacts,
    ) -> Self {
        self.impacts = Some(value);
        self
    }

    pub fn incident_type(
        mut self,
        value: crate::datadogV2::model::RelationshipToIncidentType,
    ) -> Self {
        self.incident_type = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for IncidentImportResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentImportResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImportResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentImportResponseRelationshipsVisitor {
            type Value = IncidentImportResponseRelationships;

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
                let mut declared_by_user: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut impacts: Option<crate::datadogV2::model::RelationshipToIncidentImpacts> =
                    None;
                let mut incident_type: Option<crate::datadogV2::model::RelationshipToIncidentType> =
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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "declared_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            declared_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impacts" => {
                            if v.is_null() {
                                continue;
                            }
                            impacts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_type" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentImportResponseRelationships {
                    attachments,
                    commander_user,
                    created_by_user,
                    declared_by_user,
                    impacts,
                    incident_type,
                    integrations,
                    last_modified_by_user,
                    responders,
                    user_defined_fields,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImportResponseRelationshipsVisitor)
    }
}
