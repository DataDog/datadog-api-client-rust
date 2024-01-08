// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Set the boolean property enabled to enable or disable single sign on with SAML.
/// See the SAML documentation for more information about all SAML settings.
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
}
impl Default for OrganizationSettingsSaml {
    fn default() -> Self {
        Self::new()
    }
}
