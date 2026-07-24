// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a postmortem template returned in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PostmortemTemplateResponseRelationships {
    /// Relationship to the incident type this template belongs to.
    #[serde(rename = "incident_type")]
    pub incident_type: Option<crate::datadogV2::model::PostmortemTemplateIncidentTypeRelationship>,
    /// Relationship to a user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::PostmortemTemplateUserRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PostmortemTemplateResponseRelationships {
    pub fn new() -> PostmortemTemplateResponseRelationships {
        PostmortemTemplateResponseRelationships {
            incident_type: None,
            last_modified_by_user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn incident_type(
        mut self,
        value: crate::datadogV2::model::PostmortemTemplateIncidentTypeRelationship,
    ) -> Self {
        self.incident_type = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::PostmortemTemplateUserRelationship,
    ) -> Self {
        self.last_modified_by_user = Some(value);
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

impl Default for PostmortemTemplateResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PostmortemTemplateResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PostmortemTemplateResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for PostmortemTemplateResponseRelationshipsVisitor {
            type Value = PostmortemTemplateResponseRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incident_type: Option<
                    crate::datadogV2::model::PostmortemTemplateIncidentTypeRelationship,
                > = None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::PostmortemTemplateUserRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incident_type" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PostmortemTemplateResponseRelationships {
                    incident_type,
                    last_modified_by_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PostmortemTemplateResponseRelationshipsVisitor)
    }
}
