// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a mapping created by an `add` operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchResultDataAttributes {
    /// The ID of the RUM application, when one was provided.
    #[serde(rename = "application_id")]
    pub application_id: Option<uuid::Uuid>,
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
    /// The RUM application's service name, when one was provided.
    #[serde(rename = "service")]
    pub service: Option<String>,
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

impl TeamsOwnershipMappingBatchResultDataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        match_type: crate::datadogV2::model::TeamsOwnershipMatchType,
        org_id: i64,
        team_handle: String,
        view_name: String,
    ) -> TeamsOwnershipMappingBatchResultDataAttributes {
        TeamsOwnershipMappingBatchResultDataAttributes {
            application_id: None,
            created_at,
            created_by,
            match_type,
            org_id,
            service: None,
            team_handle,
            view_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: uuid::Uuid) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchResultDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchResultDataAttributesVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchResultDataAttributesVisitor {
            type Value = TeamsOwnershipMappingBatchResultDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<uuid::Uuid> = None;
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let match_type = match_type.ok_or_else(|| M::Error::missing_field("match_type"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let team_handle =
                    team_handle.ok_or_else(|| M::Error::missing_field("team_handle"))?;
                let view_name = view_name.ok_or_else(|| M::Error::missing_field("view_name"))?;

                let content = TeamsOwnershipMappingBatchResultDataAttributes {
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

        deserializer.deserialize_any(TeamsOwnershipMappingBatchResultDataAttributesVisitor)
    }
}
