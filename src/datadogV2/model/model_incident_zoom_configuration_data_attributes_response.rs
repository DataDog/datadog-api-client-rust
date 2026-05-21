// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Zoom configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentZoomConfigurationDataAttributesResponse {
    /// Timestamp when the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether manual meeting creation is enabled.
    #[serde(rename = "manual_meeting_creation")]
    pub manual_meeting_creation: bool,
    /// Whether meeting chat timeline sync is enabled.
    #[serde(rename = "meeting_chat_timeline_sync")]
    pub meeting_chat_timeline_sync: bool,
    /// Timestamp when the configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Whether post-meeting summary is enabled.
    #[serde(rename = "post_meeting_summary")]
    pub post_meeting_summary: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentZoomConfigurationDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        manual_meeting_creation: bool,
        meeting_chat_timeline_sync: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        post_meeting_summary: bool,
    ) -> IncidentZoomConfigurationDataAttributesResponse {
        IncidentZoomConfigurationDataAttributesResponse {
            created_at,
            manual_meeting_creation,
            meeting_chat_timeline_sync,
            modified_at,
            post_meeting_summary,
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

impl<'de> Deserialize<'de> for IncidentZoomConfigurationDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentZoomConfigurationDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentZoomConfigurationDataAttributesResponseVisitor {
            type Value = IncidentZoomConfigurationDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut manual_meeting_creation: Option<bool> = None;
                let mut meeting_chat_timeline_sync: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut post_meeting_summary: Option<bool> = None;
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
                        "manual_meeting_creation" => {
                            manual_meeting_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meeting_chat_timeline_sync" => {
                            meeting_chat_timeline_sync =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "post_meeting_summary" => {
                            post_meeting_summary =
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
                let manual_meeting_creation = manual_meeting_creation
                    .ok_or_else(|| M::Error::missing_field("manual_meeting_creation"))?;
                let meeting_chat_timeline_sync = meeting_chat_timeline_sync
                    .ok_or_else(|| M::Error::missing_field("meeting_chat_timeline_sync"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let post_meeting_summary = post_meeting_summary
                    .ok_or_else(|| M::Error::missing_field("post_meeting_summary"))?;

                let content = IncidentZoomConfigurationDataAttributesResponse {
                    created_at,
                    manual_meeting_creation,
                    meeting_chat_timeline_sync,
                    modified_at,
                    post_meeting_summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentZoomConfigurationDataAttributesResponseVisitor)
    }
}
