// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Datadog-Azure integrations configured for your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureAccount {
    /// Limit the Azure app service plans that are pulled into Datadog using tags.
    /// Only app service plans that match one of the defined tags are imported into Datadog.
    #[serde(rename = "app_service_plan_filters")]
    pub app_service_plan_filters: Option<String>,
    /// Silence monitors for expected Azure VM shutdowns.
    #[serde(rename = "automute")]
    pub automute: Option<bool>,
    /// Your Azure web application ID.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// Your Azure web application secret key.
    #[serde(rename = "client_secret")]
    pub client_secret: Option<String>,
    /// Limit the Azure container apps that are pulled into Datadog using tags.
    /// Only container apps that match one of the defined tags are imported into Datadog.
    #[serde(rename = "container_app_filters")]
    pub container_app_filters: Option<String>,
    /// When enabled, Datadog’s Cloud Security Management product scans resource configurations monitored by this app registration.
    /// Note: This requires resource_collection_enabled to be set to true.
    #[serde(rename = "cspm_enabled")]
    pub cspm_enabled: Option<bool>,
    /// Enable custom metrics for your organization.
    #[serde(rename = "custom_metrics_enabled")]
    pub custom_metrics_enabled: Option<bool>,
    /// Errors in your configuration.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
    /// Limit the Azure instances that are pulled into Datadog by using tags.
    /// Only hosts that match one of the defined tags are imported into Datadog.
    #[serde(rename = "host_filters")]
    pub host_filters: Option<String>,
    /// Your New Azure web application ID.
    #[serde(rename = "new_client_id")]
    pub new_client_id: Option<String>,
    /// Your New Azure Active Directory ID.
    #[serde(rename = "new_tenant_name")]
    pub new_tenant_name: Option<String>,
    /// When enabled, Datadog collects metadata and configuration info from cloud resources (compute instances, databases, load balancers, etc.) monitored by this app registration.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    /// Your Azure Active Directory ID.
    #[serde(rename = "tenant_name")]
    pub tenant_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureAccount {
    pub fn new() -> AzureAccount {
        AzureAccount {
            app_service_plan_filters: None,
            automute: None,
            client_id: None,
            client_secret: None,
            container_app_filters: None,
            cspm_enabled: None,
            custom_metrics_enabled: None,
            errors: None,
            host_filters: None,
            new_client_id: None,
            new_tenant_name: None,
            resource_collection_enabled: None,
            tenant_name: None,
            _unparsed: false,
        }
    }

    pub fn app_service_plan_filters(mut self, value: String) -> Self {
        self.app_service_plan_filters = Some(value);
        self
    }

    pub fn automute(mut self, value: bool) -> Self {
        self.automute = Some(value);
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_secret(mut self, value: String) -> Self {
        self.client_secret = Some(value);
        self
    }

    pub fn container_app_filters(mut self, value: String) -> Self {
        self.container_app_filters = Some(value);
        self
    }

    pub fn cspm_enabled(mut self, value: bool) -> Self {
        self.cspm_enabled = Some(value);
        self
    }

    pub fn custom_metrics_enabled(mut self, value: bool) -> Self {
        self.custom_metrics_enabled = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn host_filters(mut self, value: String) -> Self {
        self.host_filters = Some(value);
        self
    }

    pub fn new_client_id(mut self, value: String) -> Self {
        self.new_client_id = Some(value);
        self
    }

    pub fn new_tenant_name(mut self, value: String) -> Self {
        self.new_tenant_name = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    pub fn tenant_name(mut self, value: String) -> Self {
        self.tenant_name = Some(value);
        self
    }
}

impl Default for AzureAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AzureAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureAccountVisitor;
        impl<'a> Visitor<'a> for AzureAccountVisitor {
            type Value = AzureAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut app_service_plan_filters: Option<String> = None;
                let mut automute: Option<bool> = None;
                let mut client_id: Option<String> = None;
                let mut client_secret: Option<String> = None;
                let mut container_app_filters: Option<String> = None;
                let mut cspm_enabled: Option<bool> = None;
                let mut custom_metrics_enabled: Option<bool> = None;
                let mut errors: Option<Vec<String>> = None;
                let mut host_filters: Option<String> = None;
                let mut new_client_id: Option<String> = None;
                let mut new_tenant_name: Option<String> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut tenant_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "app_service_plan_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            app_service_plan_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "automute" => {
                            if v.is_null() {
                                continue;
                            }
                            automute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_app_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            container_app_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cspm_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_metrics_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_metrics_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            host_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            new_client_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_tenant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            new_tenant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tenant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AzureAccount {
                    app_service_plan_filters,
                    automute,
                    client_id,
                    client_secret,
                    container_app_filters,
                    cspm_enabled,
                    custom_metrics_enabled,
                    errors,
                    host_filters,
                    new_client_id,
                    new_tenant_name,
                    resource_collection_enabled,
                    tenant_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureAccountVisitor)
    }
}
