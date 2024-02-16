// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Set the boolean property enabled to enable or disable single sign on with SAML.
/// See the SAML documentation for more information about all SAML settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSaml {
    /// Whether or not SAML is enabled for this organization.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSaml {
    pub fn new() -> OrganizationSettingsSaml {
        OrganizationSettingsSaml { enabled: None }
    }

    pub fn enabled(&mut self, value: bool) -> &mut Self {
        self.enabled = Some(value);
        self
    }
}

impl Default for OrganizationSettingsSaml {
    fn default() -> Self {
        Self::new()
    }
}
