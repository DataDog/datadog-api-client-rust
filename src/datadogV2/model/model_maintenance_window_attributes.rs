// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a maintenance window, including its schedule and the query that determines which cases are affected.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MaintenanceWindowAttributes {
    /// The UUID of the user who created this maintenance window. Read-only.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// The ISO 8601 timestamp when the maintenance window ends and normal notification behavior resumes.
    #[serde(rename = "end_at")]
    pub end_at: chrono::DateTime<chrono::Utc>,
    /// A human-readable name for the maintenance window (for example, `Database migration - Dec 15`).
    #[serde(rename = "name")]
    pub name: String,
    /// A case search query that determines which cases are affected during the maintenance window. Uses the same syntax as the Case Management search bar.
    #[serde(rename = "query")]
    pub query: String,
    /// The ISO 8601 timestamp when the maintenance window begins and notifications start being suppressed.
    #[serde(rename = "start_at")]
    pub start_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who last modified this maintenance window. Read-only.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MaintenanceWindowAttributes {
    pub fn new(
        end_at: chrono::DateTime<chrono::Utc>,
        name: String,
        query: String,
        start_at: chrono::DateTime<chrono::Utc>,
    ) -> MaintenanceWindowAttributes {
        MaintenanceWindowAttributes {
            created_by: None,
            end_at,
            name,
            query,
            start_at,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn updated_by(mut self, value: String) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for MaintenanceWindowAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaintenanceWindowAttributesVisitor;
        impl<'a> Visitor<'a> for MaintenanceWindowAttributesVisitor {
            type Value = MaintenanceWindowAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<String> = None;
                let mut end_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut start_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_at" => {
                            end_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_at" => {
                            start_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let end_at = end_at.ok_or_else(|| M::Error::missing_field("end_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let start_at = start_at.ok_or_else(|| M::Error::missing_field("start_at"))?;

                let content = MaintenanceWindowAttributes {
                    created_by,
                    end_at,
                    name,
                    query,
                    start_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MaintenanceWindowAttributesVisitor)
    }
}
