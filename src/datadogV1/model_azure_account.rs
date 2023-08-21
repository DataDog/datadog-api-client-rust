// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAccount {
    /// Silence monitors for expected Azure VM shutdowns.
    #[serde(rename = "automute", skip_serializing_if = "Option::is_none")]
    pub automute: bool,
    /// Your Azure web application ID.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: String,
    /// Your Azure web application secret key.
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: String,
    /// Errors in your configuration.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Vec<String>,
    /// Limit the Azure instances that are pulled into Datadog by using tags.
Only hosts that match one of the defined tags are imported into Datadog.
    #[serde(rename = "host_filters", skip_serializing_if = "Option::is_none")]
    pub host_filters: String,
    /// Your New Azure web application ID.
    #[serde(rename = "new_client_id", skip_serializing_if = "Option::is_none")]
    pub new_client_id: String,
    /// Your New Azure Active Directory ID.
    #[serde(rename = "new_tenant_name", skip_serializing_if = "Option::is_none")]
    pub new_tenant_name: String,
    /// Your Azure Active Directory ID.
    #[serde(rename = "tenant_name", skip_serializing_if = "Option::is_none")]
    pub tenant_name: String,
}

