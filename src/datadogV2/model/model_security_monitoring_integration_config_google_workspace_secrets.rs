// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Credentials for a Google Workspace entity context sync.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
    /// The admin email to impersonate for domain-wide delegation.
    #[serde(rename = "admin_email")]
    pub admin_email: Option<String>,
    /// The Google Cloud service account JSON used to authenticate against the Google Workspace Admin SDK. Additional keys beyond those documented are preserved.
    #[serde(rename = "service_account_json")]
    pub service_account_json:
        crate::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
    pub fn new(
        service_account_json: crate::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount,
    ) -> SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
        SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
            admin_email: None,
            service_account_json,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn admin_email(mut self, value: String) -> Self {
        self.admin_email = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationConfigGoogleWorkspaceSecretsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationConfigGoogleWorkspaceSecretsVisitor {
            type Value = SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut admin_email: Option<String> = None;
                let mut service_account_json: Option<crate::datadogV2::model::SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "admin_email" => {
                            if v.is_null() {
                                continue;
                            }
                            admin_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account_json" => {
                            service_account_json =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let service_account_json = service_account_json
                    .ok_or_else(|| M::Error::missing_field("service_account_json"))?;

                let content = SecurityMonitoringIntegrationConfigGoogleWorkspaceSecrets {
                    admin_email,
                    service_account_json,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecurityMonitoringIntegrationConfigGoogleWorkspaceSecretsVisitor)
    }
}
