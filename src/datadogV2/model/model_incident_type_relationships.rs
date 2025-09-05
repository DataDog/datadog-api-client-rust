// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident type's resource relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTypeRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// The definition of `IncidentTypeRelationshipsGoogleMeetConfiguration` object.
    #[serde(
        rename = "google_meet_configuration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub google_meet_configuration:
        Option<Option<crate::datadogV2::model::IncidentTypeRelationshipsGoogleMeetConfiguration>>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// The definition of `IncidentTypeRelationshipsMicrosoftTeamsConfiguration` object.
    #[serde(
        rename = "microsoft_teams_configuration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub microsoft_teams_configuration: Option<
        Option<crate::datadogV2::model::IncidentTypeRelationshipsMicrosoftTeamsConfiguration>,
    >,
    /// The definition of `IncidentTypeRelationshipsZoomConfiguration` object.
    #[serde(
        rename = "zoom_configuration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub zoom_configuration:
        Option<Option<crate::datadogV2::model::IncidentTypeRelationshipsZoomConfiguration>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTypeRelationships {
    pub fn new() -> IncidentTypeRelationships {
        IncidentTypeRelationships {
            created_by_user: None,
            google_meet_configuration: None,
            last_modified_by_user: None,
            microsoft_teams_configuration: None,
            zoom_configuration: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_user(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn google_meet_configuration(
        mut self,
        value: Option<crate::datadogV2::model::IncidentTypeRelationshipsGoogleMeetConfiguration>,
    ) -> Self {
        self.google_meet_configuration = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn microsoft_teams_configuration(
        mut self,
        value: Option<
            crate::datadogV2::model::IncidentTypeRelationshipsMicrosoftTeamsConfiguration,
        >,
    ) -> Self {
        self.microsoft_teams_configuration = Some(value);
        self
    }

    pub fn zoom_configuration(
        mut self,
        value: Option<crate::datadogV2::model::IncidentTypeRelationshipsZoomConfiguration>,
    ) -> Self {
        self.zoom_configuration = Some(value);
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

impl Default for IncidentTypeRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentTypeRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTypeRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentTypeRelationshipsVisitor {
            type Value = IncidentTypeRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut google_meet_configuration: Option<
                    Option<
                        crate::datadogV2::model::IncidentTypeRelationshipsGoogleMeetConfiguration,
                    >,
                > = None;
                let mut last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut microsoft_teams_configuration: Option<Option<crate::datadogV2::model::IncidentTypeRelationshipsMicrosoftTeamsConfiguration>> = None;
                let mut zoom_configuration: Option<
                    Option<crate::datadogV2::model::IncidentTypeRelationshipsZoomConfiguration>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "google_meet_configuration" => {
                            google_meet_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "microsoft_teams_configuration" => {
                            microsoft_teams_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "zoom_configuration" => {
                            zoom_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentTypeRelationships {
                    created_by_user,
                    google_meet_configuration,
                    last_modified_by_user,
                    microsoft_teams_configuration,
                    zoom_configuration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTypeRelationshipsVisitor)
    }
}
