// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Zoom meeting associated with an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentZoomMeeting {
    /// The Zoom host identifier.
    #[serde(rename = "host_id")]
    pub host_id: Option<String>,
    /// The URL to join the meeting.
    #[serde(rename = "join_url")]
    pub join_url: String,
    /// The Zoom meeting identifier.
    #[serde(rename = "meeting_id")]
    pub meeting_id: i64,
    /// The meeting password.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The URL of the meeting recording.
    #[serde(rename = "recording_url")]
    pub recording_url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentZoomMeeting {
    pub fn new(join_url: String, meeting_id: i64) -> IncidentZoomMeeting {
        IncidentZoomMeeting {
            host_id: None,
            join_url,
            meeting_id,
            password: None,
            recording_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn host_id(mut self, value: String) -> Self {
        self.host_id = Some(value);
        self
    }

    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn recording_url(mut self, value: String) -> Self {
        self.recording_url = Some(value);
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

impl<'de> Deserialize<'de> for IncidentZoomMeeting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentZoomMeetingVisitor;
        impl<'a> Visitor<'a> for IncidentZoomMeetingVisitor {
            type Value = IncidentZoomMeeting;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut host_id: Option<String> = None;
                let mut join_url: Option<String> = None;
                let mut meeting_id: Option<i64> = None;
                let mut password: Option<String> = None;
                let mut recording_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "host_id" => {
                            if v.is_null() {
                                continue;
                            }
                            host_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_url" => {
                            join_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meeting_id" => {
                            meeting_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "password" => {
                            if v.is_null() {
                                continue;
                            }
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recording_url" => {
                            if v.is_null() {
                                continue;
                            }
                            recording_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let join_url = join_url.ok_or_else(|| M::Error::missing_field("join_url"))?;
                let meeting_id = meeting_id.ok_or_else(|| M::Error::missing_field("meeting_id"))?;

                let content = IncidentZoomMeeting {
                    host_id,
                    join_url,
                    meeting_id,
                    password,
                    recording_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentZoomMeetingVisitor)
    }
}
