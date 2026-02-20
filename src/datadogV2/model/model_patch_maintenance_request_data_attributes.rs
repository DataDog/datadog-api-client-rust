// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The supported attributes for updating a maintenance.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchMaintenanceRequestDataAttributes {
    /// Timestamp of when the maintenance was completed.
    #[serde(rename = "completed_date")]
    pub completed_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The description shown when the maintenance is completed.
    #[serde(rename = "completed_description")]
    pub completed_description: Option<String>,
    /// The components affected by the maintenance.
    #[serde(rename = "components_affected")]
    pub components_affected: Option<
        Vec<crate::datadogV2::model::PatchMaintenanceRequestDataAttributesComponentsAffectedItems>,
    >,
    /// The description shown while the maintenance is in progress.
    #[serde(rename = "in_progress_description")]
    pub in_progress_description: Option<String>,
    /// The description shown when the maintenance is scheduled.
    #[serde(rename = "scheduled_description")]
    pub scheduled_description: Option<String>,
    /// Timestamp of when the maintenance is scheduled to start.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The status of the maintenance.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::MaintenanceDataAttributesStatus>,
    /// The title of the maintenance.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchMaintenanceRequestDataAttributes {
    pub fn new() -> PatchMaintenanceRequestDataAttributes {
        PatchMaintenanceRequestDataAttributes {
            completed_date: None,
            completed_description: None,
            components_affected: None,
            in_progress_description: None,
            scheduled_description: None,
            start_date: None,
            status: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn completed_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.completed_date = Some(value);
        self
    }

    pub fn completed_description(mut self, value: String) -> Self {
        self.completed_description = Some(value);
        self
    }

    pub fn components_affected(
        mut self,
        value: Vec<
            crate::datadogV2::model::PatchMaintenanceRequestDataAttributesComponentsAffectedItems,
        >,
    ) -> Self {
        self.components_affected = Some(value);
        self
    }

    pub fn in_progress_description(mut self, value: String) -> Self {
        self.in_progress_description = Some(value);
        self
    }

    pub fn scheduled_description(mut self, value: String) -> Self {
        self.scheduled_description = Some(value);
        self
    }

    pub fn start_date(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::MaintenanceDataAttributesStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for PatchMaintenanceRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchMaintenanceRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchMaintenanceRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for PatchMaintenanceRequestDataAttributesVisitor {
            type Value = PatchMaintenanceRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut completed_description: Option<String> = None;
                let mut components_affected: Option<Vec<crate::datadogV2::model::PatchMaintenanceRequestDataAttributesComponentsAffectedItems>> = None;
                let mut in_progress_description: Option<String> = None;
                let mut scheduled_description: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::MaintenanceDataAttributesStatus> =
                    None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completed_date" => {
                            if v.is_null() {
                                continue;
                            }
                            completed_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "completed_description" => {
                            if v.is_null() {
                                continue;
                            }
                            completed_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "components_affected" => {
                            if v.is_null() {
                                continue;
                            }
                            components_affected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "in_progress_description" => {
                            if v.is_null() {
                                continue;
                            }
                            in_progress_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_description" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduled_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::MaintenanceDataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PatchMaintenanceRequestDataAttributes {
                    completed_date,
                    completed_description,
                    components_affected,
                    in_progress_description,
                    scheduled_description,
                    start_date,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PatchMaintenanceRequestDataAttributesVisitor)
    }
}
