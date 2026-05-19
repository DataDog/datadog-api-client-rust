// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes required to create a maintenance window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MaintenanceWindowCreateAttributes {
    /// The end time of the maintenance window.
    #[serde(rename = "end_at")]
    pub end_at: chrono::DateTime<chrono::Utc>,
    /// The name of the maintenance window.
    #[serde(rename = "name")]
    pub name: String,
    /// The query to filter event management cases for this maintenance window.
    #[serde(rename = "query")]
    pub query: String,
    /// The start time of the maintenance window.
    #[serde(rename = "start_at")]
    pub start_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MaintenanceWindowCreateAttributes {
    pub fn new(
        end_at: chrono::DateTime<chrono::Utc>,
        name: String,
        query: String,
        start_at: chrono::DateTime<chrono::Utc>,
    ) -> MaintenanceWindowCreateAttributes {
        MaintenanceWindowCreateAttributes {
            end_at,
            name,
            query,
            start_at,
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

impl<'de> Deserialize<'de> for MaintenanceWindowCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaintenanceWindowCreateAttributesVisitor;
        impl<'a> Visitor<'a> for MaintenanceWindowCreateAttributesVisitor {
            type Value = MaintenanceWindowCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut start_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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

                let content = MaintenanceWindowCreateAttributes {
                    end_at,
                    name,
                    query,
                    start_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MaintenanceWindowCreateAttributesVisitor)
    }
}
