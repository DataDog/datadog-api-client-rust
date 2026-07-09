// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Overrides applied when activating the integration. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationActivateAttributes {
    /// The domain associated with the external entity source.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// The display name for the entity context sync configuration.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Free-form, non-sensitive settings for the entity context sync. The accepted keys depend on the source type.
    #[serde(rename = "settings")]
    pub settings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationActivateAttributes {
    pub fn new() -> SecurityMonitoringIntegrationActivateAttributes {
        SecurityMonitoringIntegrationActivateAttributes {
            domain: None,
            name: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.settings = Some(value);
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

impl Default for SecurityMonitoringIntegrationActivateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationActivateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationActivateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationActivateAttributesVisitor {
            type Value = SecurityMonitoringIntegrationActivateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<String> = None;
                let mut name: Option<String> = None;
                let mut settings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "domain" => {
                            if v.is_null() {
                                continue;
                            }
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringIntegrationActivateAttributes {
                    domain,
                    name,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringIntegrationActivateAttributesVisitor)
    }
}
