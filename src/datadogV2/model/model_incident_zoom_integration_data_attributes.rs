// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Zoom integration metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentZoomIntegrationDataAttributes {
    /// Timestamp when the integration was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The type of integration.
    #[serde(rename = "integration_type")]
    pub integration_type: String,
    /// List of Zoom meetings.
    #[serde(rename = "meetings")]
    pub meetings: Vec<crate::datadogV2::model::IncidentZoomMeeting>,
    /// Timestamp when the integration was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// The status of the integration.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentZoomIntegrationDataAttributes {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        integration_type: String,
        meetings: Vec<crate::datadogV2::model::IncidentZoomMeeting>,
        modified: chrono::DateTime<chrono::Utc>,
        status: String,
    ) -> IncidentZoomIntegrationDataAttributes {
        IncidentZoomIntegrationDataAttributes {
            created,
            integration_type,
            meetings,
            modified,
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

impl<'de> Deserialize<'de> for IncidentZoomIntegrationDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentZoomIntegrationDataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentZoomIntegrationDataAttributesVisitor {
            type Value = IncidentZoomIntegrationDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut integration_type: Option<String> = None;
                let mut meetings: Option<Vec<crate::datadogV2::model::IncidentZoomMeeting>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meetings" => {
                            meetings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;
                let meetings = meetings.ok_or_else(|| M::Error::missing_field("meetings"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = IncidentZoomIntegrationDataAttributes {
                    created,
                    integration_type,
                    meetings,
                    modified,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentZoomIntegrationDataAttributesVisitor)
    }
}
