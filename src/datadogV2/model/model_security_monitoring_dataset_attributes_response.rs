// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a Cloud SIEM dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetAttributesResponse {
    /// The creation timestamp of the dataset, in ISO 8601 format.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The Datadog handle of the user who created the dataset.
    #[serde(rename = "createdByHandle")]
    pub created_by_handle: String,
    /// The display name of the user who created the dataset.
    #[serde(rename = "createdByName")]
    pub created_by_name: String,
    /// The definition of the dataset. The shape depends on the value of `data_source`.
    /// Use `reference_table` or `managed_resource` for a referential dataset, or one of the
    /// event platform sources (for example `logs`, `audit`, `events`, `spans`, `rum`) for
    /// an event platform dataset.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
    /// The description of the dataset.
    #[serde(rename = "description")]
    pub description: String,
    /// The UUID of the dataset.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the dataset is an out-of-the-box dataset provided by Datadog.
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    /// Whether the dataset is marked as deprecated.
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    /// The timestamp of the last modification of the dataset, in ISO 8601 format.
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    /// The unique name of the dataset.
    #[serde(rename = "name")]
    pub name: String,
    /// The Datadog handle of the user who last updated the dataset.
    #[serialize_always]
    #[serde(rename = "updatedByHandle")]
    pub updated_by_handle: Option<String>,
    /// The display name of the user who last updated the dataset.
    #[serialize_always]
    #[serde(rename = "updatedByName")]
    pub updated_by_name: Option<String>,
    /// The current version of the dataset.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetAttributesResponse {
    pub fn new(
        created_at: String,
        created_by_handle: String,
        created_by_name: String,
        definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
        description: String,
        id: String,
        is_default: bool,
        is_deprecated: bool,
        modified_at: String,
        name: String,
        updated_by_handle: Option<String>,
        updated_by_name: Option<String>,
        version: i64,
    ) -> SecurityMonitoringDatasetAttributesResponse {
        SecurityMonitoringDatasetAttributesResponse {
            created_at,
            created_by_handle,
            created_by_name,
            definition,
            description,
            id,
            is_default,
            is_deprecated,
            modified_at,
            name,
            updated_by_handle,
            updated_by_name,
            version,
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetAttributesResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetAttributesResponseVisitor {
            type Value = SecurityMonitoringDatasetAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut created_by_handle: Option<String> = None;
                let mut created_by_name: Option<String> = None;
                let mut definition: Option<
                    crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
                > = None;
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut is_deprecated: Option<bool> = None;
                let mut modified_at: Option<String> = None;
                let mut name: Option<String> = None;
                let mut updated_by_handle: Option<Option<String>> = None;
                let mut updated_by_name: Option<Option<String>> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByHandle" => {
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByName" => {
                            created_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDefault" => {
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isDeprecated" => {
                            is_deprecated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modifiedAt" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedByHandle" => {
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedByName" => {
                            updated_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by_handle = created_by_handle
                    .ok_or_else(|| M::Error::missing_field("created_by_handle"))?;
                let created_by_name =
                    created_by_name.ok_or_else(|| M::Error::missing_field("created_by_name"))?;
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let is_default = is_default.ok_or_else(|| M::Error::missing_field("is_default"))?;
                let is_deprecated =
                    is_deprecated.ok_or_else(|| M::Error::missing_field("is_deprecated"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let updated_by_handle = updated_by_handle
                    .ok_or_else(|| M::Error::missing_field("updated_by_handle"))?;
                let updated_by_name =
                    updated_by_name.ok_or_else(|| M::Error::missing_field("updated_by_name"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = SecurityMonitoringDatasetAttributesResponse {
                    created_at,
                    created_by_handle,
                    created_by_name,
                    definition,
                    description,
                    id,
                    is_default,
                    is_deprecated,
                    modified_at,
                    name,
                    updated_by_handle,
                    updated_by_name,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetAttributesResponseVisitor)
    }
}
