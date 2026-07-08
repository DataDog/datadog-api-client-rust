// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the Entra ID Azure App Registration prerequisites.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
    /// The Azure App Registrations discovered for the organization.
    #[serde(rename = "azure_app_registrations")]
    pub azure_app_registrations:
        Vec<crate::datadogV2::model::SecurityMonitoringAzureAppRegistration>,
    /// Whether at least one Azure App Registration has resource collection enabled.
    #[serde(rename = "has_valid_prerequisite")]
    pub has_valid_prerequisite: bool,
    /// The ID of the Entra ID integration configuration, if one exists.
    #[serde(rename = "integration_id")]
    pub integration_id: Option<String>,
    /// Whether the Entra ID integration configuration is enabled, if one exists.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The time at which the Entra ID integration configuration was created, if one exists.
    #[serde(rename = "subscribed_at")]
    pub subscribed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
    pub fn new(
        azure_app_registrations: Vec<
            crate::datadogV2::model::SecurityMonitoringAzureAppRegistration,
        >,
        has_valid_prerequisite: bool,
    ) -> SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
        SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
            azure_app_registrations,
            has_valid_prerequisite,
            integration_id: None,
            is_enabled: None,
            subscribed_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn integration_id(mut self, value: String) -> Self {
        self.integration_id = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn subscribed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.subscribed_at = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringEntraIdAzureAppRegistrationsAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringEntraIdAzureAppRegistrationsAttributesVisitor {
            type Value = SecurityMonitoringEntraIdAzureAppRegistrationsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_app_registrations: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringAzureAppRegistration>,
                > = None;
                let mut has_valid_prerequisite: Option<bool> = None;
                let mut integration_id: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut subscribed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_app_registrations" => {
                            azure_app_registrations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_valid_prerequisite" => {
                            has_valid_prerequisite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_id" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscribed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            subscribed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let azure_app_registrations = azure_app_registrations
                    .ok_or_else(|| M::Error::missing_field("azure_app_registrations"))?;
                let has_valid_prerequisite = has_valid_prerequisite
                    .ok_or_else(|| M::Error::missing_field("has_valid_prerequisite"))?;

                let content = SecurityMonitoringEntraIdAzureAppRegistrationsAttributes {
                    azure_app_registrations,
                    has_valid_prerequisite,
                    integration_id,
                    is_enabled,
                    subscribed_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecurityMonitoringEntraIdAzureAppRegistrationsAttributesVisitor)
    }
}
