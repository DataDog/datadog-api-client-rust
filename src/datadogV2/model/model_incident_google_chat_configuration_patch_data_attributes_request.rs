// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for patching a Google Chat configuration. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleChatConfigurationPatchDataAttributesRequest {
    /// The Google Chat domain ID.
    #[serde(rename = "domain_id")]
    pub domain_id: Option<String>,
    /// The template for the Google Chat space name.
    #[serde(rename = "space_name_template")]
    pub space_name_template: Option<String>,
    /// The target audience ID for the Google Chat space.
    #[serde(rename = "space_target_audience_id")]
    pub space_target_audience_id: Option<String>,
    /// The time zone for the Google Chat space.
    #[serde(rename = "space_time_zone")]
    pub space_time_zone: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleChatConfigurationPatchDataAttributesRequest {
    pub fn new() -> IncidentGoogleChatConfigurationPatchDataAttributesRequest {
        IncidentGoogleChatConfigurationPatchDataAttributesRequest {
            domain_id: None,
            space_name_template: None,
            space_target_audience_id: None,
            space_time_zone: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn domain_id(mut self, value: String) -> Self {
        self.domain_id = Some(value);
        self
    }

    pub fn space_name_template(mut self, value: String) -> Self {
        self.space_name_template = Some(value);
        self
    }

    pub fn space_target_audience_id(mut self, value: String) -> Self {
        self.space_target_audience_id = Some(value);
        self
    }

    pub fn space_time_zone(mut self, value: String) -> Self {
        self.space_time_zone = Some(value);
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

impl Default for IncidentGoogleChatConfigurationPatchDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentGoogleChatConfigurationPatchDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleChatConfigurationPatchDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleChatConfigurationPatchDataAttributesRequestVisitor {
            type Value = IncidentGoogleChatConfigurationPatchDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain_id: Option<String> = None;
                let mut space_name_template: Option<String> = None;
                let mut space_target_audience_id: Option<String> = None;
                let mut space_time_zone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain_id" => {
                            if v.is_null() {
                                continue;
                            }
                            domain_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_name_template" => {
                            if v.is_null() {
                                continue;
                            }
                            space_name_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_target_audience_id" => {
                            if v.is_null() {
                                continue;
                            }
                            space_target_audience_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_time_zone" => {
                            if v.is_null() {
                                continue;
                            }
                            space_time_zone =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentGoogleChatConfigurationPatchDataAttributesRequest {
                    domain_id,
                    space_name_template,
                    space_target_audience_id,
                    space_time_zone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(IncidentGoogleChatConfigurationPatchDataAttributesRequestVisitor)
    }
}
