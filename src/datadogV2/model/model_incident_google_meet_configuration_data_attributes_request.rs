// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Google Meet configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleMeetConfigurationDataAttributesRequest {
    /// Whether to allow manual meeting creation.
    #[serde(rename = "allow_manual_meeting_creation")]
    pub allow_manual_meeting_creation: bool,
    /// Whether to auto-summarize meetings.
    #[serde(rename = "auto_summarize")]
    pub auto_summarize: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleMeetConfigurationDataAttributesRequest {
    pub fn new(
        allow_manual_meeting_creation: bool,
        auto_summarize: bool,
    ) -> IncidentGoogleMeetConfigurationDataAttributesRequest {
        IncidentGoogleMeetConfigurationDataAttributesRequest {
            allow_manual_meeting_creation,
            auto_summarize,
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

impl<'de> Deserialize<'de> for IncidentGoogleMeetConfigurationDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleMeetConfigurationDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleMeetConfigurationDataAttributesRequestVisitor {
            type Value = IncidentGoogleMeetConfigurationDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_manual_meeting_creation: Option<bool> = None;
                let mut auto_summarize: Option<bool> = None;
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

                let content = IncidentGoogleMeetConfigurationDataAttributesRequest {
                    allow_manual_meeting_creation,
                    auto_summarize,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentGoogleMeetConfigurationDataAttributesRequestVisitor)
    }
}
