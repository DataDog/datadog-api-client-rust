// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for patching a Google Meet configuration. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
    /// Whether to allow manual meeting creation.
    #[serde(rename = "allow_manual_meeting_creation")]
    pub allow_manual_meeting_creation: Option<bool>,
    /// Whether to auto-summarize meetings.
    #[serde(rename = "auto_summarize")]
    pub auto_summarize: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
    pub fn new() -> IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
        IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
            allow_manual_meeting_creation: None,
            auto_summarize: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_manual_meeting_creation(mut self, value: bool) -> Self {
        self.allow_manual_meeting_creation = Some(value);
        self
    }

    pub fn auto_summarize(mut self, value: bool) -> Self {
        self.auto_summarize = Some(value);
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

impl Default for IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleMeetConfigurationPatchDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleMeetConfigurationPatchDataAttributesRequestVisitor {
            type Value = IncidentGoogleMeetConfigurationPatchDataAttributesRequest;

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
                            if v.is_null() {
                                continue;
                            }
                            allow_manual_meeting_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auto_summarize" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = IncidentGoogleMeetConfigurationPatchDataAttributesRequest {
                    allow_manual_meeting_creation,
                    auto_summarize,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(IncidentGoogleMeetConfigurationPatchDataAttributesRequestVisitor)
    }
}
