// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Your Google Cloud Platform Account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    /// Limit the Cloud Run revisions that are pulled into Datadog by using tags.
    /// Only Cloud Run revision resources that apply to specified filters are imported into Datadog.
    #[serde(rename = "cloud_run_revision_filters")]
    pub cloud_run_revision_filters: Option<Vec<String>>,
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
    /// When enabled, Datadog scans for all resource change data in your Google Cloud environment.
    #[serde(rename = "is_resource_change_collection_enabled")]
    pub is_resource_change_collection_enabled: Option<bool>,
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            cloud_run_revision_filters: None,
            errors: None,
            host_filters: None,
            is_cspm_enabled: None,
            is_resource_change_collection_enabled: None,
            is_security_command_center_enabled: None,
            private_key: None,
            private_key_id: None,
            project_id: None,
            resource_collection_enabled: None,
            token_uri: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth_provider_x509_cert_url(mut self, value: String) -> Self {
        self.auth_provider_x509_cert_url = Some(value);
        self
    }

    pub fn auth_uri(mut self, value: String) -> Self {
        self.auth_uri = Some(value);
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

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_x509_cert_url(mut self, value: String) -> Self {
        self.client_x509_cert_url = Some(value);
        self
    }

    pub fn cloud_run_revision_filters(mut self, value: Vec<String>) -> Self {
        self.cloud_run_revision_filters = Some(value);
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

    pub fn is_cspm_enabled(mut self, value: bool) -> Self {
        self.is_cspm_enabled = Some(value);
        self
    }

    pub fn is_resource_change_collection_enabled(mut self, value: bool) -> Self {
        self.is_resource_change_collection_enabled = Some(value);
        self
    }

    pub fn is_security_command_center_enabled(mut self, value: bool) -> Self {
        self.is_security_command_center_enabled = Some(value);
        self
    }

    pub fn private_key(mut self, value: String) -> Self {
        self.private_key = Some(value);
        self
    }

    pub fn private_key_id(mut self, value: String) -> Self {
        self.private_key_id = Some(value);
        self
    }

    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }

    pub fn resource_collection_enabled(mut self, value: bool) -> Self {
        self.resource_collection_enabled = Some(value);
        self
    }

    pub fn token_uri(mut self, value: String) -> Self {
        self.token_uri = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for GCPAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPAccountVisitor;
        impl<'a> Visitor<'a> for GCPAccountVisitor {
            type Value = GCPAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_provider_x509_cert_url: Option<String> = None;
                let mut auth_uri: Option<String> = None;
                let mut automute: Option<bool> = None;
                let mut client_email: Option<String> = None;
                let mut client_id: Option<String> = None;
                let mut client_x509_cert_url: Option<String> = None;
                let mut cloud_run_revision_filters: Option<Vec<String>> = None;
                let mut errors: Option<Vec<String>> = None;
                let mut host_filters: Option<String> = None;
                let mut is_cspm_enabled: Option<bool> = None;
                let mut is_resource_change_collection_enabled: Option<bool> = None;
                let mut is_security_command_center_enabled: Option<bool> = None;
                let mut private_key: Option<String> = None;
                let mut private_key_id: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut resource_collection_enabled: Option<bool> = None;
                let mut token_uri: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_provider_x509_cert_url" => {
                            if v.is_null() {
                                continue;
                            }
                            auth_provider_x509_cert_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auth_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            auth_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "automute" => {
                            if v.is_null() {
                                continue;
                            }
                            automute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_email" => {
                            if v.is_null() {
                                continue;
                            }
                            client_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_x509_cert_url" => {
                            if v.is_null() {
                                continue;
                            }
                            client_x509_cert_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_run_revision_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_run_revision_filters =
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
                        "is_cspm_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_cspm_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_resource_change_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_resource_change_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_security_command_center_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_security_command_center_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key" => {
                            if v.is_null() {
                                continue;
                            }
                            private_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            private_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_collection_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_collection_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_uri" => {
                            if v.is_null() {
                                continue;
                            }
                            token_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GCPAccount {
                    auth_provider_x509_cert_url,
                    auth_uri,
                    automute,
                    client_email,
                    client_id,
                    client_x509_cert_url,
                    cloud_run_revision_filters,
                    errors,
                    host_filters,
                    is_cspm_enabled,
                    is_resource_change_collection_enabled,
                    is_security_command_center_enabled,
                    private_key,
                    private_key_id,
                    project_id,
                    resource_collection_enabled,
                    token_uri,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPAccountVisitor)
    }
}
