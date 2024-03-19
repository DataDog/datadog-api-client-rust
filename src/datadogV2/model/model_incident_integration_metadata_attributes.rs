// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident integration metadata's attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentIntegrationMetadataAttributes {
    /// Timestamp when the incident todo was created.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the incident this integration metadata is connected to.
    #[serde(rename = "incident_id")]
    pub incident_id: Option<String>,
    /// A number indicating the type of integration this metadata is for. 1 indicates Slack;
    /// 8 indicates Jira.
    #[serde(rename = "integration_type")]
    pub integration_type: i32,
    /// Incident integration metadata's metadata attribute.
    #[serde(rename = "metadata")]
    pub metadata: crate::datadogV2::model::IncidentIntegrationMetadataMetadata,
    /// Timestamp when the incident todo was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// A number indicating the status of this integration metadata. 0 indicates unknown;
    /// 1 indicates pending; 2 indicates complete; 3 indicates manually created;
    /// 4 indicates manually updated; 5 indicates failed.
    #[serde(rename = "status")]
    pub status: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentIntegrationMetadataAttributes {
    pub fn new(
        integration_type: i32,
        metadata: crate::datadogV2::model::IncidentIntegrationMetadataMetadata,
    ) -> IncidentIntegrationMetadataAttributes {
        IncidentIntegrationMetadataAttributes {
            created: None,
            incident_id: None,
            integration_type,
            metadata,
            modified: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn incident_id(mut self, value: String) -> Self {
        self.incident_id = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn status(mut self, value: i32) -> Self {
        self.status = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentIntegrationMetadataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentIntegrationMetadataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentIntegrationMetadataAttributesVisitor {
            type Value = IncidentIntegrationMetadataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut incident_id: Option<String> = None;
                let mut integration_type: Option<i32> = None;
                let mut metadata: Option<
                    crate::datadogV2::model::IncidentIntegrationMetadataMetadata,
                > = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metadata) = metadata {
                                match _metadata {
                                    crate::datadogV2::model::IncidentIntegrationMetadataMetadata::UnparsedObject(_metadata) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;

                let content = IncidentIntegrationMetadataAttributes {
                    created,
                    incident_id,
                    integration_type,
                    metadata,
                    modified,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentIntegrationMetadataAttributesVisitor)
    }
}
