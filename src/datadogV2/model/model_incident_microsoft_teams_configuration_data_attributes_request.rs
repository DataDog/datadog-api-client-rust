// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a Microsoft Teams configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
    /// Whether manual meeting creation is enabled.
    #[serde(rename = "manual_meeting_creation")]
    pub manual_meeting_creation: Option<bool>,
    /// Whether post-meeting summary is enabled.
    #[serde(rename = "post_meeting_summary")]
    pub post_meeting_summary: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
    pub fn new() -> IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
        IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
            manual_meeting_creation: None,
            post_meeting_summary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn manual_meeting_creation(mut self, value: bool) -> Self {
        self.manual_meeting_creation = Some(value);
        self
    }

    pub fn post_meeting_summary(mut self, value: bool) -> Self {
        self.post_meeting_summary = Some(value);
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

impl Default for IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentMicrosoftTeamsConfigurationDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentMicrosoftTeamsConfigurationDataAttributesRequestVisitor {
            type Value = IncidentMicrosoftTeamsConfigurationDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut manual_meeting_creation: Option<bool> = None;
                let mut post_meeting_summary: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "manual_meeting_creation" => {
                            if v.is_null() {
                                continue;
                            }
                            manual_meeting_creation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "post_meeting_summary" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = IncidentMicrosoftTeamsConfigurationDataAttributesRequest {
                    manual_meeting_creation,
                    post_meeting_summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(IncidentMicrosoftTeamsConfigurationDataAttributesRequestVisitor)
    }
}
