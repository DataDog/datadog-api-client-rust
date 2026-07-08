// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fields to update on the entity context sync configuration. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationConfigUpdateAttributes {
    /// The new domain associated with the external entity source.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Whether the entity context sync should be enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The type of external source that provides entities to Cloud SIEM.
    #[serde(rename = "integration_type")]
    pub integration_type: Option<crate::datadogV2::model::SecurityMonitoringIntegrationType>,
    /// The new display name for the entity context sync configuration.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The secrets used to authenticate against the external entity source. The accepted keys depend on the source type
    /// (for example, `admin_email` for Google Workspace). Not required for source types that do not use secrets (for example, `ENTRA_ID`).
    #[serde(rename = "secrets")]
    pub secrets: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Free-form, non-sensitive settings for the entity context sync. The accepted keys depend on the source type.
    #[serde(rename = "settings")]
    pub settings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationConfigUpdateAttributes {
    pub fn new() -> SecurityMonitoringIntegrationConfigUpdateAttributes {
        SecurityMonitoringIntegrationConfigUpdateAttributes {
            domain: None,
            enabled: None,
            integration_type: None,
            name: None,
            secrets: None,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn domain(mut self, value: String) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn integration_type(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringIntegrationType,
    ) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn secrets(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.secrets = Some(value);
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

impl Default for SecurityMonitoringIntegrationConfigUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationConfigUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationConfigUpdateAttributesVisitor {
            type Value = SecurityMonitoringIntegrationConfigUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut integration_type: Option<
                    crate::datadogV2::model::SecurityMonitoringIntegrationType,
                > = None;
                let mut name: Option<String> = None;
                let mut secrets: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
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
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration_type) = integration_type {
                                match _integration_type {
                                    crate::datadogV2::model::SecurityMonitoringIntegrationType::UnparsedObject(_integration_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secrets" => {
                            if v.is_null() {
                                continue;
                            }
                            secrets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SecurityMonitoringIntegrationConfigUpdateAttributes {
                    domain,
                    enabled,
                    integration_type,
                    name,
                    secrets,
                    settings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringIntegrationConfigUpdateAttributesVisitor)
    }
}
