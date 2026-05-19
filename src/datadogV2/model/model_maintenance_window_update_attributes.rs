// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that can be updated on a maintenance window. All fields are optional; only provided fields are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MaintenanceWindowUpdateAttributes {
    /// The end time of the maintenance window.
    #[serde(rename = "end_at")]
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the maintenance window.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query to filter event management cases for this maintenance window.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The start time of the maintenance window.
    #[serde(rename = "start_at")]
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MaintenanceWindowUpdateAttributes {
    pub fn new() -> MaintenanceWindowUpdateAttributes {
        MaintenanceWindowUpdateAttributes {
            end_at: None,
            name: None,
            query: None,
            start_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn start_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_at = Some(value);
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

impl Default for MaintenanceWindowUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MaintenanceWindowUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaintenanceWindowUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for MaintenanceWindowUpdateAttributesVisitor {
            type Value = MaintenanceWindowUpdateAttributes;

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
                            if v.is_null() {
                                continue;
                            }
                            end_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_at" => {
                            if v.is_null() {
                                continue;
                            }
                            start_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MaintenanceWindowUpdateAttributes {
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

        deserializer.deserialize_any(MaintenanceWindowUpdateAttributesVisitor)
    }
}
