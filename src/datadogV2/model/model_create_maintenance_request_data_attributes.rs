// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The supported attributes for creating a maintenance.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateMaintenanceRequestDataAttributes {
    /// Timestamp of when the maintenance was completed.
    #[serde(rename = "completed_date")]
    pub completed_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The description shown when the maintenance is completed.
    #[serde(rename = "completed_description")]
    pub completed_description: Option<String>,
    /// The components affected by the maintenance.
    #[serde(rename = "components_affected")]
    pub components_affected:
        Vec<crate::datadogV2::model::CreateMaintenanceRequestDataAttributesComponentsAffectedItems>,
    /// The description shown while the maintenance is in progress.
    #[serde(rename = "in_progress_description")]
    pub in_progress_description: Option<String>,
    /// The description shown when the maintenance is scheduled.
    #[serde(rename = "scheduled_description")]
    pub scheduled_description: Option<String>,
    /// Timestamp of when the maintenance is scheduled to start.
    #[serde(rename = "start_date")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The title of the maintenance.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateMaintenanceRequestDataAttributes {
    pub fn new(
        components_affected: Vec<
            crate::datadogV2::model::CreateMaintenanceRequestDataAttributesComponentsAffectedItems,
        >,
        title: String,
    ) -> CreateMaintenanceRequestDataAttributes {
        CreateMaintenanceRequestDataAttributes {
            completed_date: None,
            completed_description: None,
            components_affected,
            in_progress_description: None,
            scheduled_description: None,
            start_date: None,
            title,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CreateMaintenanceRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateMaintenanceRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateMaintenanceRequestDataAttributesVisitor {
            type Value = CreateMaintenanceRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut completed_description: Option<String> = None;
                let mut components_affected: Option<Vec<crate::datadogV2::model::CreateMaintenanceRequestDataAttributesComponentsAffectedItems>> = None;
                let mut in_progress_description: Option<String> = None;
                let mut scheduled_description: Option<String> = None;
                let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let components_affected = components_affected
                    .ok_or_else(|| M::Error::missing_field("components_affected"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = CreateMaintenanceRequestDataAttributes {
                    completed_date,
                    completed_description,
                    components_affected,
                    in_progress_description,
                    scheduled_description,
                    start_date,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateMaintenanceRequestDataAttributesVisitor)
    }
}
