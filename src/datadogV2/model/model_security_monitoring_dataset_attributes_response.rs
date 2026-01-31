// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a security monitoring dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetAttributesResponse {
    /// The creation timestamp of the dataset.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The handle of the user who created the dataset.
    #[serde(rename = "createdByHandle")]
    pub created_by_handle: Option<String>,
    /// The name of the user who created the dataset.
    #[serde(rename = "createdByName")]
    pub created_by_name: Option<String>,
    /// The definition of a dataset, including its data source, name, indexes, and columns.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
    /// A description of the dataset.
    #[serde(rename = "description")]
    pub description: String,
    /// The last modification timestamp of the dataset.
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the dataset.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization ID.
    #[serde(rename = "orgId")]
    pub org_id: i64,
    /// The handle of the user who last updated the dataset.
    #[serde(rename = "updatedByHandle")]
    pub updated_by_handle: Option<String>,
    /// The name of the user who last updated the dataset.
    #[serde(rename = "updatedByName")]
    pub updated_by_name: Option<String>,
    /// The version of the dataset.
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
        created_at: chrono::DateTime<chrono::Utc>,
        definition: crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
        description: String,
        name: String,
        org_id: i64,
        version: i64,
    ) -> SecurityMonitoringDatasetAttributesResponse {
        SecurityMonitoringDatasetAttributesResponse {
            created_at,
            created_by_handle: None,
            created_by_name: None,
            definition,
            description,
            modified_at: None,
            name,
            org_id,
            updated_by_handle: None,
            updated_by_name: None,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_handle(mut self, value: String) -> Self {
        self.created_by_handle = Some(value);
        self
    }

    pub fn created_by_name(mut self, value: String) -> Self {
        self.created_by_name = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn updated_by_handle(mut self, value: String) -> Self {
        self.updated_by_handle = Some(value);
        self
    }

    pub fn updated_by_name(mut self, value: String) -> Self {
        self.updated_by_name = Some(value);
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
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by_handle: Option<String> = None;
                let mut created_by_name: Option<String> = None;
                let mut definition: Option<
                    crate::datadogV2::model::SecurityMonitoringDatasetDefinition,
                > = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut updated_by_handle: Option<String> = None;
                let mut updated_by_name: Option<String> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByName" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "modifiedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "orgId" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedByHandle" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedByName" => {
                            if v.is_null() {
                                continue;
                            }
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
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = SecurityMonitoringDatasetAttributesResponse {
                    created_at,
                    created_by_handle,
                    created_by_name,
                    definition,
                    description,
                    modified_at,
                    name,
                    org_id,
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
