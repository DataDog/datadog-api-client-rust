// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes associated with your service account.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// When enabled, Datadog performs configuration checks across your Google Cloud environment by continuously scanning every resource.
    #[serde(rename = "is_cspm_enabled")]
    pub is_cspm_enabled: Option<bool>,
}

impl GCPSTSServiceAccountAttributes {
    pub fn new() -> GCPSTSServiceAccountAttributes {
        GCPSTSServiceAccountAttributes {
            account_tags: None,
            automute: None,
            client_email: None,
            host_filters: None,
            is_cspm_enabled: None,
        }
    }
}
