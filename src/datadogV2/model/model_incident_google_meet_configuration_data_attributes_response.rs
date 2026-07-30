// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Google Meet configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleMeetConfigurationDataAttributesResponse {
    /// Whether manual meeting creation is allowed.
    #[serde(rename = "allow_manual_meeting_creation")]
    pub allow_manual_meeting_creation: bool,
    /// Whether meetings are auto-summarized.
    #[serde(rename = "auto_summarize")]
    pub auto_summarize: bool,
    /// Timestamp when the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp when the configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleMeetConfigurationDataAttributesResponse {
    pub fn new(
        allow_manual_meeting_creation: bool,
        auto_summarize: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
    ) -> IncidentGoogleMeetConfigurationDataAttributesResponse {
        IncidentGoogleMeetConfigurationDataAttributesResponse {
            allow_manual_meeting_creation,
            auto_summarize,
            created_at: None,
            modified_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
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

impl<'de> Deserialize<'de> for IncidentGoogleMeetConfigurationDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleMeetConfigurationDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleMeetConfigurationDataAttributesResponseVisitor {
            type Value = IncidentGoogleMeetConfigurationDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_manual_meeting_creation: Option<bool> = None;
                let mut auto_summarize: Option<bool> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_manual_meeting_creation" => {
                            allow_manual_meeting_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auto_summarize" => {
                            auto_summarize =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allow_manual_meeting_creation = allow_manual_meeting_creation
                    .ok_or_else(|| M::Error::missing_field("allow_manual_meeting_creation"))?;
                let auto_summarize =
                    auto_summarize.ok_or_else(|| M::Error::missing_field("auto_summarize"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;

                let content = IncidentGoogleMeetConfigurationDataAttributesResponse {
                    allow_manual_meeting_creation,
                    auto_summarize,
                    created_at,
                    modified_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentGoogleMeetConfigurationDataAttributesResponseVisitor)
    }
}
