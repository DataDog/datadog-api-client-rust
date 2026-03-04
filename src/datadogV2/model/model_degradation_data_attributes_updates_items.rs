// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DegradationDataAttributesUpdatesItems {
    /// The components affected at the time of the update.
    #[serde(rename = "components_affected")]
    pub components_affected: Option<
        Vec<crate::datadogV2::model::DegradationDataAttributesUpdatesItemsComponentsAffectedItems>,
    >,
    /// Timestamp of when the update was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the update.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Identifier of the update.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// Timestamp of when the update was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the update started.
    #[serde(rename = "started_at")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The status of the degradation.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DegradationDataAttributesUpdatesItems {
    pub fn new() -> DegradationDataAttributesUpdatesItems {
        DegradationDataAttributesUpdatesItems {
            components_affected: None,
            created_at: None,
            description: None,
            id: None,
            modified_at: None,
            started_at: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components_affected(
        mut self,
        value: Vec<
            crate::datadogV2::model::DegradationDataAttributesUpdatesItemsComponentsAffectedItems,
        >,
    ) -> Self {
        self.components_affected = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn started_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
    ) -> Self {
        self.status = Some(value);
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

impl Default for DegradationDataAttributesUpdatesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DegradationDataAttributesUpdatesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DegradationDataAttributesUpdatesItemsVisitor;
        impl<'a> Visitor<'a> for DegradationDataAttributesUpdatesItemsVisitor {
            type Value = DegradationDataAttributesUpdatesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components_affected: Option<Vec<crate::datadogV2::model::DegradationDataAttributesUpdatesItemsComponentsAffectedItems>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<
                    crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components_affected" => {
                            if v.is_null() {
                                continue;
                            }
                            components_affected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DegradationDataAttributesUpdatesItems {
                    components_affected,
                    created_at,
                    description,
                    id,
                    modified_at,
                    started_at,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DegradationDataAttributesUpdatesItemsVisitor)
    }
}
