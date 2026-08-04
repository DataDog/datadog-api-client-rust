// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a maintenance template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MaintenanceTemplateDataAttributes {
    /// The description shown when a maintenance created from this template is completed.
    #[serde(rename = "completed_description")]
    pub completed_description: Option<String>,
    /// The IDs of the components affected by a maintenance created from this template.
    #[serde(rename = "component_ids")]
    pub component_ids: Option<Vec<String>>,
    /// Timestamp of when the maintenance template was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The description shown while a maintenance created from this template is in progress.
    #[serde(rename = "in_progress_description")]
    pub in_progress_description: Option<String>,
    /// The title used for a maintenance created from this template.
    #[serde(rename = "maintenance_title")]
    pub maintenance_title: Option<String>,
    /// Timestamp of when the maintenance template was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the maintenance template.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The description shown when a maintenance created from this template is scheduled.
    #[serde(rename = "scheduled_description")]
    pub scheduled_description: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MaintenanceTemplateDataAttributes {
    pub fn new() -> MaintenanceTemplateDataAttributes {
        MaintenanceTemplateDataAttributes {
            completed_description: None,
            component_ids: None,
            created_at: None,
            in_progress_description: None,
            maintenance_title: None,
            modified_at: None,
            name: None,
            scheduled_description: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn completed_description(mut self, value: String) -> Self {
        self.completed_description = Some(value);
        self
    }

    pub fn component_ids(mut self, value: Vec<String>) -> Self {
        self.component_ids = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn in_progress_description(mut self, value: String) -> Self {
        self.in_progress_description = Some(value);
        self
    }

    pub fn maintenance_title(mut self, value: String) -> Self {
        self.maintenance_title = Some(value);
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

    pub fn scheduled_description(mut self, value: String) -> Self {
        self.scheduled_description = Some(value);
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

impl Default for MaintenanceTemplateDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MaintenanceTemplateDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaintenanceTemplateDataAttributesVisitor;
        impl<'a> Visitor<'a> for MaintenanceTemplateDataAttributesVisitor {
            type Value = MaintenanceTemplateDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_description: Option<String> = None;
                let mut component_ids: Option<Vec<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut in_progress_description: Option<String> = None;
                let mut maintenance_title: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut scheduled_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completed_description" => {
                            if v.is_null() {
                                continue;
                            }
                            completed_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "component_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            component_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "in_progress_description" => {
                            if v.is_null() {
                                continue;
                            }
                            in_progress_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maintenance_title" => {
                            if v.is_null() {
                                continue;
                            }
                            maintenance_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
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
                        "scheduled_description" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduled_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MaintenanceTemplateDataAttributes {
                    completed_description,
                    component_ids,
                    created_at,
                    in_progress_description,
                    maintenance_title,
                    modified_at,
                    name,
                    scheduled_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MaintenanceTemplateDataAttributesVisitor)
    }
}
