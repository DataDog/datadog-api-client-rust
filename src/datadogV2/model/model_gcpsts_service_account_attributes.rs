// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes associated with your service account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountAttributes {
    /// Tags to be associated with GCP metrics and service checks from your account.
    #[serde(rename = "account_tags")]
    pub account_tags: Option<Vec<String>>,
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute")]
    pub automute: Option<bool>,
    /// Your service account email address.
    #[serde(rename = "client_email")]
    pub client_email: Option<String>,
    /// Your Host Filters.
    #[serde(rename = "host_filters")]
    pub host_filters: Option<Vec<String>>,
    /// When enabled, Datadog will activate the Cloud Security Monitoring product for this service account. Note: This requires resource_collection_enabled to be set to true.
    #[serde(rename = "is_cspm_enabled")]
    pub is_cspm_enabled: Option<bool>,
    /// When enabled, Datadog will attempt to collect Security Command Center Findings. Note: This requires additional permissions on the service account.
    #[serde(rename = "is_security_command_center_enabled")]
    pub is_security_command_center_enabled: Option<bool>,
    /// When enabled, Datadog scans for all resources in your GCP environment.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
}

impl GCPSTSServiceAccountAttributes {
    pub fn new() -> GCPSTSServiceAccountAttributes {
        GCPSTSServiceAccountAttributes {
            account_tags: None,
            automute: None,
            client_email: None,
            host_filters: None,
            is_cspm_enabled: None,
            is_security_command_center_enabled: None,
            resource_collection_enabled: None,
        }
    }

    pub fn account_tags(mut self, value: Vec<String>) -> Self {
        self.account_tags = Some(value);
        self
    }

    pub fn automute(mut self, value: bool) -> Self {
        self.automute = Some(value);
        self
    }

    pub fn client_email(mut self, value: String) -> Self {
        self.client_email = Some(value);
        self
    }

    pub fn host_filters(mut self, value: Vec<String>) -> Self {
        self.host_filters = Some(value);
        self
    }

    pub fn is_cspm_enabled(mut self, value: bool) -> Self {
        self.is_cspm_enabled = Some(value);
        self
    }

    pub fn is_security_command_center_enabled(mut self, value: bool) -> Self {
        self.is_security_command_center_enabled = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }
}

impl Default for GCPSTSServiceAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}
