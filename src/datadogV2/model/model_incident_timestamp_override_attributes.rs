// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident timestamp override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimestampOverrideAttributes {
    /// Timestamp of when the override was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp of when the override was deleted (soft delete).
    #[serialize_always]
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The UUID of the incident.
    #[serde(rename = "incident_id")]
    pub incident_id: uuid::Uuid,
    /// Timestamp of when the override was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The type of timestamp being overridden.
    #[serde(rename = "timestamp_type")]
    pub timestamp_type: crate::datadogV2::model::TimestampType,
    /// The timestamp value for the override.
    #[serde(rename = "timestamp_value")]
    pub timestamp_value: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimestampOverrideAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
        incident_id: uuid::Uuid,
        modified_at: chrono::DateTime<chrono::Utc>,
        timestamp_type: crate::datadogV2::model::TimestampType,
        timestamp_value: chrono::DateTime<chrono::Utc>,
    ) -> IncidentTimestampOverrideAttributes {
        IncidentTimestampOverrideAttributes {
            created_at,
            deleted_at,
            incident_id,
            modified_at,
            timestamp_type,
            timestamp_value,
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

impl<'de> Deserialize<'de> for IncidentTimestampOverrideAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimestampOverrideAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentTimestampOverrideAttributesVisitor {
            type Value = IncidentTimestampOverrideAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut incident_id: Option<uuid::Uuid> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut timestamp_type: Option<crate::datadogV2::model::TimestampType> = None;
                let mut timestamp_value: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_type" => {
                            timestamp_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timestamp_type) = timestamp_type {
                                match _timestamp_type {
                                    crate::datadogV2::model::TimestampType::UnparsedObject(
                                        _timestamp_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "timestamp_value" => {
                            timestamp_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let deleted_at = deleted_at.ok_or_else(|| M::Error::missing_field("deleted_at"))?;
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let timestamp_type =
                    timestamp_type.ok_or_else(|| M::Error::missing_field("timestamp_type"))?;
                let timestamp_value =
                    timestamp_value.ok_or_else(|| M::Error::missing_field("timestamp_value"))?;

                let content = IncidentTimestampOverrideAttributes {
                    created_at,
                    deleted_at,
                    incident_id,
                    modified_at,
                    timestamp_type,
                    timestamp_value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimestampOverrideAttributesVisitor)
    }
}
