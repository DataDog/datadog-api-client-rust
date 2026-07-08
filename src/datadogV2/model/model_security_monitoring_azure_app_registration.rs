// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An Azure App Registration discovered for the organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringAzureAppRegistration {
    /// The client ID of the App Registration.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The number of errors encountered while crawling resources for this App Registration.
    #[serde(rename = "error_count")]
    pub error_count: i64,
    /// Whether resource collection is enabled for this App Registration.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: bool,
    /// The number of Azure subscriptions associated with this App Registration.
    #[serde(rename = "subscription_count")]
    pub subscription_count: i64,
    /// The Azure tenant ID of the App Registration.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringAzureAppRegistration {
    pub fn new(
        client_id: String,
        error_count: i64,
        resource_collection_enabled: bool,
        subscription_count: i64,
        tenant_id: String,
    ) -> SecurityMonitoringAzureAppRegistration {
        SecurityMonitoringAzureAppRegistration {
            client_id,
            error_count,
            resource_collection_enabled,
            subscription_count,
            tenant_id,
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

impl<'de> Deserialize<'de> for SecurityMonitoringAzureAppRegistration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringAzureAppRegistrationVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringAzureAppRegistrationVisitor {
            type Value = SecurityMonitoringAzureAppRegistration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<String> = None;
                let mut error_count: Option<i64> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut subscription_count: Option<i64> = None;
                let mut tenant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_count" => {
                            error_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscription_count" => {
                            subscription_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let error_count =
                    error_count.ok_or_else(|| M::Error::missing_field("error_count"))?;
                let resource_collection_enabled = resource_collection_enabled
                    .ok_or_else(|| M::Error::missing_field("resource_collection_enabled"))?;
                let subscription_count = subscription_count
                    .ok_or_else(|| M::Error::missing_field("subscription_count"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = SecurityMonitoringAzureAppRegistration {
                    client_id,
                    error_count,
                    resource_collection_enabled,
                    subscription_count,
                    tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringAzureAppRegistrationVisitor)
    }
}
