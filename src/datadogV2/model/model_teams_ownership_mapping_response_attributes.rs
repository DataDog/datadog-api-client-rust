// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a teams ownership mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingResponseAttributes {
    /// The ID of the RUM application this mapping applies to.
    /// For browser applications, this is the real application UUID.
    /// For mobile applications, this is the nil UUID `00000000-0000-0000-0000-000000000000` (wildcard), meaning the ownership applies across all applications.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// Timestamp when the mapping was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the mapping.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// How the `view_name` is matched against RUM view names.
    #[serde(rename = "match_type")]
    pub match_type: crate::datadogV2::model::TeamsOwnershipMatchType,
    /// The ID of the organization that owns this mapping.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The RUM application's service name. For browser applications, may be empty. For mobile applications, this is the service that scopes the ownership.
    #[serde(rename = "service")]
    pub service: String,
    /// The handle of the team that owns the matched RUM views.
    #[serde(rename = "team_handle")]
    pub team_handle: String,
    /// The RUM view name to match, or its prefix when `match_type` is `prefix`.
    #[serde(rename = "view_name")]
    pub view_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingResponseAttributes {
    pub fn new(
        application_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        match_type: crate::datadogV2::model::TeamsOwnershipMatchType,
        org_id: i64,
        service: String,
        team_handle: String,
        view_name: String,
    ) -> TeamsOwnershipMappingResponseAttributes {
        TeamsOwnershipMappingResponseAttributes {
            application_id,
            created_at,
            created_by,
            match_type,
            org_id,
            service,
            team_handle,
            view_name,
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

impl<'de> Deserialize<'de> for TeamsOwnershipMappingResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingResponseAttributesVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingResponseAttributesVisitor {
            type Value = TeamsOwnershipMappingResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut match_type: Option<crate::datadogV2::model::TeamsOwnershipMatchType> = None;
                let mut org_id: Option<i64> = None;
                let mut service: Option<String> = None;
                let mut team_handle: Option<String> = None;
                let mut view_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_type" => {
                            match_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _match_type) = match_type {
                                match _match_type {
                                    crate::datadogV2::model::TeamsOwnershipMatchType::UnparsedObject(_match_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_handle" => {
                            team_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let match_type = match_type.ok_or_else(|| M::Error::missing_field("match_type"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let team_handle =
                    team_handle.ok_or_else(|| M::Error::missing_field("team_handle"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = TeamsOwnershipMappingResponseAttributes {
                    application_id,
                    created_at,
                    created_by,
                    match_type,
                    org_id,
                    service,
                    team_handle,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingResponseAttributesVisitor)
    }
}
