// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Google Cloud service account JSON used to authenticate against the Google Workspace Admin SDK. Additional keys beyond those documented are preserved.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
    /// The service account client email.
    #[serde(rename = "client_email")]
    pub client_email: String,
    /// The service account private key.
    #[serde(rename = "private_key")]
    pub private_key: String,
    /// The Google Cloud project ID that owns the service account.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The service account type. Must be `service_account`.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
    pub fn new(
        client_email: String,
        private_key: String,
        project_id: String,
        type_: String,
    ) -> SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
        SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
            client_email,
            private_key,
            project_id,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccountVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccountVisitor {
            type Value = SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_email: Option<String> = None;
                let mut private_key: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_email" => {
                            client_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key" => {
                            private_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_email =
                    client_email.ok_or_else(|| M::Error::missing_field("client_email"))?;
                let private_key =
                    private_key.ok_or_else(|| M::Error::missing_field("private_key"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccount {
                    client_email,
                    private_key,
                    project_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            SecurityMonitoringIntegrationConfigGoogleWorkspaceServiceAccountVisitor,
        )
    }
}
