// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Google Chat configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentGoogleChatConfigurationDataAttributesRequest {
    /// The Google Chat domain ID.
    #[serde(rename = "domain_id")]
    pub domain_id: String,
    /// The template for the Google Chat space name.
    #[serde(rename = "space_name_template")]
    pub space_name_template: String,
    /// The target audience ID for the Google Chat space.
    #[serde(rename = "space_target_audience_id")]
    pub space_target_audience_id: String,
    /// The time zone for the Google Chat space.
    #[serde(rename = "space_time_zone")]
    pub space_time_zone: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentGoogleChatConfigurationDataAttributesRequest {
    pub fn new(
        domain_id: String,
        space_name_template: String,
        space_target_audience_id: String,
        space_time_zone: String,
    ) -> IncidentGoogleChatConfigurationDataAttributesRequest {
        IncidentGoogleChatConfigurationDataAttributesRequest {
            domain_id,
            space_name_template,
            space_target_audience_id,
            space_time_zone,
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

impl<'de> Deserialize<'de> for IncidentGoogleChatConfigurationDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentGoogleChatConfigurationDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentGoogleChatConfigurationDataAttributesRequestVisitor {
            type Value = IncidentGoogleChatConfigurationDataAttributesRequest;

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
                            domain_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_name_template" => {
                            space_name_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_target_audience_id" => {
                            space_target_audience_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "space_time_zone" => {
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
                let domain_id = domain_id.ok_or_else(|| M::Error::missing_field("domain_id"))?;
                let space_name_template = space_name_template
                    .ok_or_else(|| M::Error::missing_field("space_name_template"))?;
                let space_target_audience_id = space_target_audience_id
                    .ok_or_else(|| M::Error::missing_field("space_target_audience_id"))?;
                let space_time_zone =
                    space_time_zone.ok_or_else(|| M::Error::missing_field("space_time_zone"))?;

                let content = IncidentGoogleChatConfigurationDataAttributesRequest {
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

        deserializer.deserialize_any(IncidentGoogleChatConfigurationDataAttributesRequestVisitor)
    }
}
