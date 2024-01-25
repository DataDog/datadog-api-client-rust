// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Has one property enabled (boolean).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlStrictMode {
    /// Whether or not the SAML strict mode is enabled. If true, all users must log in with SAML.
    /// Learn more on the [SAML Strict documentation](<https://docs.datadoghq.com/account_management/saml/#saml-strict>).
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSamlStrictMode {
    pub fn new() -> OrganizationSettingsSamlStrictMode {
        OrganizationSettingsSamlStrictMode { enabled: None }
    }
}
impl Default for OrganizationSettingsSamlStrictMode {
    fn default() -> Self {
        Self::new()
    }
}
