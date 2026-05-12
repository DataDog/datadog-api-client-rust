// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A degradation update entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateDegradationRequestDataAttributesUpdatesItems {
    /// The components affected.
    #[serde(rename = "components_affected")]
    pub components_affected: Vec<crate::datadogV2::model::CreateDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems>,
    /// A description of the update.
    #[serde(rename = "description")]
    pub description: String,
    /// Timestamp of when the update occurred.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The status of the degradation.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CreateDegradationRequestDataAttributesUpdatesItems {
    pub fn new(
        components_affected: Vec<crate::datadogV2::model::CreateDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems>,
        description: String,
        started_at: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::CreateDegradationRequestDataAttributesStatus,
    ) -> CreateDegradationRequestDataAttributesUpdatesItems {
        CreateDegradationRequestDataAttributesUpdatesItems {
            components_affected,
            description,
            started_at,
            status,
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

impl<'de> Deserialize<'de> for CreateDegradationRequestDataAttributesUpdatesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateDegradationRequestDataAttributesUpdatesItemsVisitor;
        impl<'a> Visitor<'a> for CreateDegradationRequestDataAttributesUpdatesItemsVisitor {
            type Value = CreateDegradationRequestDataAttributesUpdatesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components_affected: Option<Vec<crate::datadogV2::model::CreateDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems>> = None;
                let mut description: Option<String> = None;
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
                            components_affected =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
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
                let components_affected = components_affected
                    .ok_or_else(|| M::Error::missing_field("components_affected"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = CreateDegradationRequestDataAttributesUpdatesItems {
                    components_affected,
                    description,
                    started_at,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateDegradationRequestDataAttributesUpdatesItemsVisitor)
    }
}
