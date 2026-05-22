// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an entity context sync configuration as returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationConfigAttributes {
    /// The time at which the entity context sync configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The domain associated with the external entity source (for example, the customer's identity provider domain).
    #[serde(rename = "domain")]
    pub domain: String,
    /// Whether the sync is enabled and actively ingesting entities into Cloud SIEM.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The type of external source that provides entities to Cloud SIEM.
    #[serde(rename = "integration_type")]
    pub integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationType,
    /// The time at which the entity context sync configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The display name of the entity context sync configuration.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Free-form, non-sensitive settings for the entity context sync. The accepted keys depend on the source type.
    #[serde(rename = "settings")]
    pub settings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The state of the credentials configured on the entity context sync.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::SecurityMonitoringIntegrationConfigState>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationConfigAttributes {
    pub fn new(
        domain: String,
        enabled: bool,
        integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationType,
    ) -> SecurityMonitoringIntegrationConfigAttributes {
        SecurityMonitoringIntegrationConfigAttributes {
            created_at: None,
            domain,
            enabled,
            integration_type,
            modified_at: None,
            name: None,
            settings: None,
            state: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
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

    pub fn state(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringIntegrationConfigState,
    ) -> Self {
        self.state = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationConfigAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationConfigAttributesVisitor {
            type Value = SecurityMonitoringIntegrationConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut domain: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut integration_type: Option<
                    crate::datadogV2::model::SecurityMonitoringIntegrationType,
                > = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut settings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut state: Option<
                    crate::datadogV2::model::SecurityMonitoringIntegrationConfigState,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain" => {
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
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
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::SecurityMonitoringIntegrationConfigState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let domain = domain.ok_or_else(|| M::Error::missing_field("domain"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;

                let content = SecurityMonitoringIntegrationConfigAttributes {
                    created_at,
                    domain,
                    enabled,
                    integration_type,
                    modified_at,
                    name,
                    settings,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringIntegrationConfigAttributesVisitor)
    }
}
