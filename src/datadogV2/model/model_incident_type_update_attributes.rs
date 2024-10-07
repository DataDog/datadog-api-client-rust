// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident type's attributes for updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTypeUpdateAttributes {
    /// Timestamp when the incident type was created.
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A unique identifier that represents the user that created the incident type.
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    /// Text that describes the incident type.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// When true, this incident type will be used as the default type when an incident type is not specified.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// A unique identifier that represents the user that last modified the incident type.
    #[serde(rename = "lastModifiedBy")]
    pub last_modified_by: Option<String>,
    /// Timestamp when the incident type was last modified.
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the incident type.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The string that will be prepended to the incident title across the Datadog app.
    #[serde(rename = "prefix")]
    pub prefix: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTypeUpdateAttributes {
    pub fn new() -> IncidentTypeUpdateAttributes {
        IncidentTypeUpdateAttributes {
            created_at: None,
            created_by: None,
            description: None,
            is_default: None,
            last_modified_by: None,
            modified_at: None,
            name: None,
            prefix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn last_modified_by(mut self, value: String) -> Self {
        self.last_modified_by = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn prefix(mut self, value: String) -> Self {
        self.prefix = Some(value);
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

impl Default for IncidentTypeUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentTypeUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTypeUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentTypeUpdateAttributesVisitor {
            type Value = IncidentTypeUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut last_modified_by: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdBy" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lastModifiedBy" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by =
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
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentTypeUpdateAttributes {
                    created_at,
                    created_by,
                    description,
                    is_default,
                    last_modified_by,
                    modified_at,
                    name,
                    prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTypeUpdateAttributesVisitor)
    }
}
