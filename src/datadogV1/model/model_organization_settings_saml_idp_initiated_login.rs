// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Has one property enabled (boolean).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlIdpInitiatedLogin {
    /// Whether SAML IdP initiated login is enabled, learn more
    /// in the [SAML documentation](<https://docs.datadoghq.com/account_management/saml/#idp-initiated-login>).
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSamlIdpInitiatedLogin {
    pub fn new() -> OrganizationSettingsSamlIdpInitiatedLogin {
        OrganizationSettingsSamlIdpInitiatedLogin { enabled: None }
    }

    pub fn enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OrganizationSettingsSamlIdpInitiatedLogin {
    fn default() -> Self {
        Self::new()
    }
}
