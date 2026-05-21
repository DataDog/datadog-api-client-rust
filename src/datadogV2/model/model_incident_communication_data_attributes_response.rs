// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident communication response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCommunicationDataAttributesResponse {
    /// The kind of communication.
    #[serde(rename = "communication_type")]
    pub communication_type: crate::datadogV2::model::IncidentCommunicationKind,
    /// The content of a communication.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentCommunicationContent,
    /// Timestamp when the communication was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The incident identifier.
    #[serde(rename = "incident_id")]
    pub incident_id: uuid::Uuid,
    /// Timestamp when the communication was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCommunicationDataAttributesResponse {
    pub fn new(
        communication_type: crate::datadogV2::model::IncidentCommunicationKind,
        content: crate::datadogV2::model::IncidentCommunicationContent,
        created: chrono::DateTime<chrono::Utc>,
        incident_id: uuid::Uuid,
        modified: chrono::DateTime<chrono::Utc>,
    ) -> IncidentCommunicationDataAttributesResponse {
        IncidentCommunicationDataAttributesResponse {
            communication_type,
            content,
            created,
            incident_id,
            modified,
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

impl<'de> Deserialize<'de> for IncidentCommunicationDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCommunicationDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentCommunicationDataAttributesResponseVisitor {
            type Value = IncidentCommunicationDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut communication_type: Option<
                    crate::datadogV2::model::IncidentCommunicationKind,
                > = None;
                let mut content: Option<crate::datadogV2::model::IncidentCommunicationContent> =
                    None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut incident_id: Option<uuid::Uuid> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "communication_type" => {
                            communication_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _communication_type) = communication_type {
                                match _communication_type {
                                    crate::datadogV2::model::IncidentCommunicationKind::UnparsedObject(_communication_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let communication_type = communication_type
                    .ok_or_else(|| M::Error::missing_field("communication_type"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;

                let content = IncidentCommunicationDataAttributesResponse {
                    communication_type,
                    content,
                    created,
                    incident_id,
                    modified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCommunicationDataAttributesResponseVisitor)
    }
}
