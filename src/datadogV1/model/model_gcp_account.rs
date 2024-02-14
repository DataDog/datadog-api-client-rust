// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Your Google Cloud Platform Account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPAccount {
    /// Should be `<https://www.googleapis.com/oauth2/v1/certs`.>
    #[serde(rename = "auth_provider_x509_cert_url")]
    pub auth_provider_x509_cert_url: Option<String>,
    /// Should be `<https://accounts.google.com/o/oauth2/auth`.>
    #[serde(rename = "auth_uri")]
    pub auth_uri: Option<String>,
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute")]
    pub automute: Option<bool>,
    /// Your email found in your JSON service account key.
    #[serde(rename = "client_email")]
    pub client_email: Option<String>,
    /// Your ID found in your JSON service account key.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// Should be `<https://www.googleapis.com/robot/v1/metadata/x509/$CLIENT_EMAIL`>
    /// where `$CLIENT_EMAIL` is the email found in your JSON service account key.
    #[serde(rename = "client_x509_cert_url")]
    pub client_x509_cert_url: Option<String>,
    /// An array of errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
    /// Limit the GCE instances that are pulled into Datadog by using tags.
    /// Only hosts that match one of the defined tags are imported into Datadog.
    #[serde(rename = "host_filters")]
    pub host_filters: Option<String>,
    /// When enabled, Datadog will activate the Cloud Security Monitoring product for this service account. Note: This requires resource_collection_enabled to be set to true.
    #[serde(rename = "is_cspm_enabled")]
    pub is_cspm_enabled: Option<bool>,
    /// When enabled, Datadog will attempt to collect Security Command Center Findings. Note: This requires additional permissions on the service account.
    #[serde(rename = "is_security_command_center_enabled")]
    pub is_security_command_center_enabled: Option<bool>,
    /// Your private key name found in your JSON service account key.
    #[serde(rename = "private_key")]
    pub private_key: Option<String>,
    /// Your private key ID found in your JSON service account key.
    #[serde(rename = "private_key_id")]
    pub private_key_id: Option<String>,
    /// Your Google Cloud project ID found in your JSON service account key.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    /// When enabled, Datadog scans for all resources in your GCP environment.
    #[serde(rename = "resource_collection_enabled")]
    pub resource_collection_enabled: Option<bool>,
    /// Should be `<https://accounts.google.com/o/oauth2/token`.>
    #[serde(rename = "token_uri")]
    pub token_uri: Option<String>,
    /// The value for service_account found in your JSON service account key.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl GCPAccount {
    pub fn new() -> GCPAccount {
        GCPAccount {
            auth_provider_x509_cert_url: None,
            auth_uri: None,
            automute: None,
            client_email: None,
            client_id: None,
            client_x509_cert_url: None,
            errors: None,
            host_filters: None,
            is_cspm_enabled: None,
            is_security_command_center_enabled: None,
            private_key: None,
            private_key_id: None,
            project_id: None,
            resource_collection_enabled: None,
            token_uri: None,
            type_: None,
        }
    }

    pub fn auth_provider_x509_cert_url(&mut self, value: String) -> &mut Self {
        self.auth_provider_x509_cert_url = Some(value);
        self
    }

    pub fn auth_uri(&mut self, value: String) -> &mut Self {
        self.auth_uri = Some(value);
        self
    }

    pub fn automute(&mut self, value: bool) -> &mut Self {
        self.automute = Some(value);
        self
    }

    pub fn client_email(&mut self, value: String) -> &mut Self {
        self.client_email = Some(value);
        self
    }

    pub fn client_id(&mut self, value: String) -> &mut Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_x509_cert_url(&mut self, value: String) -> &mut Self {
        self.client_x509_cert_url = Some(value);
        self
    }

    pub fn errors(&mut self, value: Vec<String>) -> &mut Self {
        self.errors = Some(value);
        self
    }

    pub fn host_filters(&mut self, value: String) -> &mut Self {
        self.host_filters = Some(value);
        self
    }

    pub fn is_cspm_enabled(&mut self, value: bool) -> &mut Self {
        self.is_cspm_enabled = Some(value);
        self
    }

    pub fn is_security_command_center_enabled(&mut self, value: bool) -> &mut Self {
        self.is_security_command_center_enabled = Some(value);
        self
    }

    pub fn private_key(&mut self, value: String) -> &mut Self {
        self.private_key = Some(value);
        self
    }

    pub fn private_key_id(&mut self, value: String) -> &mut Self {
        self.private_key_id = Some(value);
        self
    }

    pub fn project_id(&mut self, value: String) -> &mut Self {
        self.project_id = Some(value);
        self
    }

    pub fn resource_collection_enabled(&mut self, value: bool) -> &mut Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    pub fn token_uri(&mut self, value: String) -> &mut Self {
        self.token_uri = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for GCPAccount {
    fn default() -> Self {
        Self::new()
    }
}
