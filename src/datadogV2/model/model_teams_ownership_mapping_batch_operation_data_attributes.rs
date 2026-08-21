// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the mapping to add. `team_handle` and `view_name` are required
/// when `op` is `add`. At least one of `service` or `application_id` must be provided.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipMappingBatchOperationDataAttributes {
    /// The ID of the RUM application this mapping applies to.
    /// For browser applications, provide the real application UUID — the team is applied to the view regardless of service.
    /// For mobile applications, omit this field (or set it to the nil UUID `00000000-0000-0000-0000-000000000000`) — the team is applied to the view and service combination across all applications.
    #[serde(rename = "application_id")]
    pub application_id: Option<uuid::Uuid>,
    /// How the `view_name` is matched against RUM view names.
    #[serde(rename = "match_type")]
    pub match_type: Option<crate::datadogV2::model::TeamsOwnershipMatchType>,
    /// The RUM application's service name. For browser applications, this is optional. For mobile applications, this is required and scopes the ownership to a specific service.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The handle of the team that owns the matched RUM views.
    #[serde(rename = "team_handle")]
    pub team_handle: Option<String>,
    /// The RUM view name to match, or its prefix when `match_type` is `prefix`.
    #[serde(rename = "view_name")]
    pub view_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipMappingBatchOperationDataAttributes {
    pub fn new() -> TeamsOwnershipMappingBatchOperationDataAttributes {
        TeamsOwnershipMappingBatchOperationDataAttributes {
            application_id: None,
            match_type: None,
            service: None,
            team_handle: None,
            view_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: uuid::Uuid) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn match_type(mut self, value: crate::datadogV2::model::TeamsOwnershipMatchType) -> Self {
        self.match_type = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn team_handle(mut self, value: String) -> Self {
        self.team_handle = Some(value);
        self
    }

    pub fn view_name(mut self, value: String) -> Self {
        self.view_name = Some(value);
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

impl Default for TeamsOwnershipMappingBatchOperationDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamsOwnershipMappingBatchOperationDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipMappingBatchOperationDataAttributesVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipMappingBatchOperationDataAttributesVisitor {
            type Value = TeamsOwnershipMappingBatchOperationDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<uuid::Uuid> = None;
                let mut match_type: Option<crate::datadogV2::model::TeamsOwnershipMatchType> = None;
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
                        "match_type" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            team_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            if v.is_null() {
                                continue;
                            }
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamsOwnershipMappingBatchOperationDataAttributes {
                    application_id,
                    match_type,
                    service,
                    team_handle,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipMappingBatchOperationDataAttributesVisitor)
    }
}
