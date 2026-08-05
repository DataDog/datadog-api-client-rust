// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes for creating a maintenance template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateMaintenanceTemplateRequestDataAttributes {
    /// The description shown when a maintenance created from this template is completed.
    #[serde(rename = "completed_description")]
    pub completed_description: Option<String>,
    /// The IDs of the components affected by a maintenance created from this template.
    #[serde(rename = "component_ids")]
    pub component_ids: Option<Vec<String>>,
    /// The description shown while a maintenance created from this template is in progress.
    #[serde(rename = "in_progress_description")]
    pub in_progress_description: Option<String>,
    /// The title used for a maintenance created from this template.
    #[serde(rename = "maintenance_title")]
    pub maintenance_title: Option<String>,
    /// The name of the maintenance template.
    #[serde(rename = "name")]
    pub name: String,
    /// The description shown when a maintenance created from this template is scheduled.
    #[serde(rename = "scheduled_description")]
    pub scheduled_description: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateMaintenanceTemplateRequestDataAttributes {
    pub fn new(name: String) -> CreateMaintenanceTemplateRequestDataAttributes {
        CreateMaintenanceTemplateRequestDataAttributes {
            completed_description: None,
            component_ids: None,
            in_progress_description: None,
            maintenance_title: None,
            name,
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

    pub fn in_progress_description(mut self, value: String) -> Self {
        self.in_progress_description = Some(value);
        self
    }

    pub fn maintenance_title(mut self, value: String) -> Self {
        self.maintenance_title = Some(value);
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

impl<'de> Deserialize<'de> for CreateMaintenanceTemplateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateMaintenanceTemplateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateMaintenanceTemplateRequestDataAttributesVisitor {
            type Value = CreateMaintenanceTemplateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_description: Option<String> = None;
                let mut component_ids: Option<Vec<String>> = None;
                let mut in_progress_description: Option<String> = None;
                let mut maintenance_title: Option<String> = None;
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
                        "name" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CreateMaintenanceTemplateRequestDataAttributes {
                    completed_description,
                    component_ids,
                    in_progress_description,
                    maintenance_title,
                    name,
                    scheduled_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateMaintenanceTemplateRequestDataAttributesVisitor)
    }
}
