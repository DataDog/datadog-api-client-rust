// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a degradation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DegradationDataAttributes {
    /// Components affected by the degradation.
    #[serde(rename = "components_affected")]
    pub components_affected:
        Option<Vec<crate::datadogV2::model::DegradationDataAttributesComponentsAffectedItems>>,
    /// Timestamp of when the degradation was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the degradation.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Timestamp of when the degradation was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The source of the degradation.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::DegradationDataAttributesSource>,
    /// The status of the degradation.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus>,
    /// Title of the degradation.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Past updates made to the degradation.
    #[serde(rename = "updates")]
    pub updates: Option<Vec<crate::datadogV2::model::DegradationDataAttributesUpdatesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DegradationDataAttributes {
    pub fn new() -> DegradationDataAttributes {
        DegradationDataAttributes {
            components_affected: None,
            created_at: None,
            description: None,
            modified_at: None,
            source: None,
            status: None,
            title: None,
            updates: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components_affected(
        mut self,
        value: Vec<crate::datadogV2::model::DegradationDataAttributesComponentsAffectedItems>,
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

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn source(
        mut self,
        value: crate::datadogV2::model::DegradationDataAttributesSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn updates(
        mut self,
        value: Vec<crate::datadogV2::model::DegradationDataAttributesUpdatesItems>,
    ) -> Self {
        self.updates = Some(value);
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

impl Default for DegradationDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DegradationDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DegradationDataAttributesVisitor;
        impl<'a> Visitor<'a> for DegradationDataAttributesVisitor {
            type Value = DegradationDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components_affected: Option<
                    Vec<crate::datadogV2::model::DegradationDataAttributesComponentsAffectedItems>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut source: Option<crate::datadogV2::model::DegradationDataAttributesSource> =
                    None;
                let mut status: Option<
                    crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
                > = None;
                let mut title: Option<String> = None;
                let mut updates: Option<
                    Vec<crate::datadogV2::model::DegradationDataAttributesUpdatesItems>,
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updates" => {
                            if v.is_null() {
                                continue;
                            }
                            updates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DegradationDataAttributes {
                    components_affected,
                    created_at,
                    description,
                    modified_at,
                    source,
                    status,
                    title,
                    updates,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DegradationDataAttributesVisitor)
    }
}
