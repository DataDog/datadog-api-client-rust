// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the entity context sync configuration to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationConfigCreateAttributes {
    /// The domain associated with the external entity source.
    #[serde(rename = "domain")]
    pub domain: String,
    /// The type of external source that provides entities to Cloud SIEM.
    #[serde(rename = "integration_type")]
    pub integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationType,
    /// The display name for the entity context sync configuration.
    #[serde(rename = "name")]
    pub name: String,
    /// The secrets used to authenticate against the external entity source. The accepted keys depend on the source type (for example, `admin_email` for Google Workspace).
    #[serde(rename = "secrets")]
    pub secrets: std::collections::BTreeMap<String, serde_json::Value>,
    /// Free-form, non-sensitive settings for the entity context sync. The accepted keys depend on the source type.
    #[serde(rename = "settings")]
    pub settings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationConfigCreateAttributes {
    pub fn new(
        domain: String,
        integration_type: crate::datadogV2::model::SecurityMonitoringIntegrationType,
        name: String,
        secrets: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> SecurityMonitoringIntegrationConfigCreateAttributes {
        SecurityMonitoringIntegrationConfigCreateAttributes {
            domain,
            integration_type,
            name,
            secrets,
            settings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationConfigCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationConfigCreateAttributesVisitor {
            type Value = SecurityMonitoringIntegrationConfigCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut domain: Option<String> = None;
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
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secrets" => {
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
                let domain = domain.ok_or_else(|| M::Error::missing_field("domain"))?;
                let integration_type =
                    integration_type.ok_or_else(|| M::Error::missing_field("integration_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let secrets = secrets.ok_or_else(|| M::Error::missing_field("secrets"))?;

                let content = SecurityMonitoringIntegrationConfigCreateAttributes {
                    domain,
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

        deserializer.deserialize_any(SecurityMonitoringIntegrationConfigCreateAttributesVisitor)
    }
}
