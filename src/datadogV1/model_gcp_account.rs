// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPAccount {
    /// Should be `https://www.googleapis.com/oauth2/v1/certs`.
    #[serde(rename = "auth_provider_x509_cert_url", skip_serializing_if = "Option::is_none")]
    pub auth_provider_x509_cert_url: String,
    /// Should be `https://accounts.google.com/o/oauth2/auth`.
    #[serde(rename = "auth_uri", skip_serializing_if = "Option::is_none")]
    pub auth_uri: String,
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute", skip_serializing_if = "Option::is_none")]
    pub automute: bool,
    /// Your email found in your JSON service account key.
    #[serde(rename = "client_email", skip_serializing_if = "Option::is_none")]
    pub client_email: String,
    /// Your ID found in your JSON service account key.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: String,
    /// Should be `https://www.googleapis.com/robot/v1/metadata/x509/$CLIENT_EMAIL`
where `$CLIENT_EMAIL` is the email found in your JSON service account key.
    #[serde(rename = "client_x509_cert_url", skip_serializing_if = "Option::is_none")]
    pub client_x509_cert_url: String,
    /// An array of errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Vec<String>,
    /// Limit the GCE instances that are pulled into Datadog by using tags.
Only hosts that match one of the defined tags are imported into Datadog.
    #[serde(rename = "host_filters", skip_serializing_if = "Option::is_none")]
    pub host_filters: String,
    /// When enabled, Datadog performs configuration checks across your Google Cloud environment by continuously scanning every resource.
    #[serde(rename = "is_cspm_enabled", skip_serializing_if = "Option::is_none")]
    pub is_cspm_enabled: bool,
    /// Your private key name found in your JSON service account key.
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: String,
    /// Your private key ID found in your JSON service account key.
    #[serde(rename = "private_key_id", skip_serializing_if = "Option::is_none")]
    pub private_key_id: String,
    /// Your Google Cloud project ID found in your JSON service account key.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: String,
    /// Should be `https://accounts.google.com/o/oauth2/token`.
    #[serde(rename = "token_uri", skip_serializing_if = "Option::is_none")]
    pub token_uri: String,
    /// The value for service_account found in your JSON service account key.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

