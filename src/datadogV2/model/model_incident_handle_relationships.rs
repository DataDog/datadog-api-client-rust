// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentHandleRelationships {
    #[serde(rename = "commander_user")]
    pub commander_user: Option<crate::datadogV2::model::IncidentHandleRelationship>,
    #[serde(rename = "created_by_user")]
    pub created_by_user: crate::datadogV2::model::IncidentHandleRelationship,
    #[serde(rename = "incident_type")]
    pub incident_type: crate::datadogV2::model::IncidentHandleRelationship,
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: crate::datadogV2::model::IncidentHandleRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentHandleRelationships {
    pub fn new(
        created_by_user: crate::datadogV2::model::IncidentHandleRelationship,
        incident_type: crate::datadogV2::model::IncidentHandleRelationship,
        last_modified_by_user: crate::datadogV2::model::IncidentHandleRelationship,
    ) -> IncidentHandleRelationships {
        IncidentHandleRelationships {
            commander_user: None,
            created_by_user,
            incident_type,
            last_modified_by_user,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn commander_user(
        mut self,
        value: crate::datadogV2::model::IncidentHandleRelationship,
    ) -> Self {
        self.commander_user = Some(value);
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

impl<'de> Deserialize<'de> for IncidentHandleRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentHandleRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentHandleRelationshipsVisitor {
            type Value = IncidentHandleRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commander_user: Option<
                    crate::datadogV2::model::IncidentHandleRelationship,
                > = None;
                let mut created_by_user: Option<
                    crate::datadogV2::model::IncidentHandleRelationship,
                > = None;
                let mut incident_type: Option<crate::datadogV2::model::IncidentHandleRelationship> =
                    None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::IncidentHandleRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commander_user" => {
                            if v.is_null() {
                                continue;
                            }
                            commander_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_user" => {
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_type" => {
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
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
                let created_by_user =
                    created_by_user.ok_or_else(|| M::Error::missing_field("created_by_user"))?;
                let incident_type =
                    incident_type.ok_or_else(|| M::Error::missing_field("incident_type"))?;
                let last_modified_by_user = last_modified_by_user
                    .ok_or_else(|| M::Error::missing_field("last_modified_by_user"))?;

                let content = IncidentHandleRelationships {
                    commander_user,
                    created_by_user,
                    incident_type,
                    last_modified_by_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentHandleRelationshipsVisitor)
    }
}
