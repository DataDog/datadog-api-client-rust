// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Has two properties, `enabled` (boolean) and `domains`, which is a list of domains without the @ symbol.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlAutocreateUsersDomains {
    /// List of domains where the SAML automated user creation is enabled.
    #[serde(rename = "domains")]
    pub domains: Option<Vec<String>>,
    /// Whether or not the automated user creation based on SAML domain is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSamlAutocreateUsersDomains {
    pub fn new() -> OrganizationSettingsSamlAutocreateUsersDomains {
        OrganizationSettingsSamlAutocreateUsersDomains {
            domains: None,
            enabled: None,
        }
    }
}