// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Datadog-Azure integrations configured for your organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// Enable Cloud Security Management Misconfigurations for your organization.
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
    /// Your Azure Active Directory ID.
    #[serde(rename = "tenant_name")]
    pub tenant_name: Option<String>,
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
            tenant_name: None,
        }
    }
}
