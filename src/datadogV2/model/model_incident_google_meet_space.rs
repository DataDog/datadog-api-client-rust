// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Google Meet space associated with an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleMeetSpace {
    /// The URL to join the Google Meet space.
    #[serde(rename = "join_url")]
    pub join_url: String,
    /// The meeting code for the space.
    #[serde(rename = "meeting_code")]
    pub meeting_code: String,
    /// The name of the Google Meet space.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleMeetSpace {
    pub fn new(join_url: String, meeting_code: String, name: String) -> IncidentGoogleMeetSpace {
        IncidentGoogleMeetSpace {
            join_url,
            meeting_code,
            name,
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

impl<'de> Deserialize<'de> for IncidentGoogleMeetSpace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleMeetSpaceVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleMeetSpaceVisitor {
            type Value = IncidentGoogleMeetSpace;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut join_url: Option<String> = None;
                let mut meeting_code: Option<String> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "join_url" => {
                            join_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meeting_code" => {
                            meeting_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let join_url = join_url.ok_or_else(|| M::Error::missing_field("join_url"))?;
                let meeting_code =
                    meeting_code.ok_or_else(|| M::Error::missing_field("meeting_code"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = IncidentGoogleMeetSpace {
                    join_url,
                    meeting_code,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentGoogleMeetSpaceVisitor)
    }
}
