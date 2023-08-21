// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountAttributes {
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute", skip_serializing_if = "Option::is_none")]
    pub automute: bool,
    /// Your service account email address.
    #[serde(rename = "client_email", skip_serializing_if = "Option::is_none")]
    pub client_email: String,
    /// Your Host Filters.
    #[serde(rename = "host_filters", skip_serializing_if = "Option::is_none")]
    pub host_filters: Vec<String>,
    /// When enabled, Datadog performs configuration checks across your Google Cloud environment by continuously scanning every resource.
    #[serde(rename = "is_cspm_enabled", skip_serializing_if = "Option::is_none")]
    pub is_cspm_enabled: bool,
}

