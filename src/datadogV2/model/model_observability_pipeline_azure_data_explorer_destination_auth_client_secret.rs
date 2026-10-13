// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authenticate using a Microsoft Entra application client secret.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
    /// The Microsoft Entra application (client) ID.
    #[serde(rename = "azure_client_id")]
    pub azure_client_id: String,
    /// Name of the environment variable or secret that holds the Microsoft Entra application client secret.
    #[serde(rename = "azure_client_secret_key")]
    pub azure_client_secret_key: String,
    /// The Azure credential kind. The value should always be `client_secret_credential`.
    #[serde(rename = "azure_credential_kind")]
    pub azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretKind,
    /// The Microsoft Entra tenant ID.
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
    pub fn new(
        azure_client_id: String,
        azure_client_secret_key: String,
        azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretKind,
        azure_tenant_id: String,
    ) -> ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
        ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
            azure_client_id,
            azure_client_secret_key,
            azure_credential_kind,
            azure_tenant_id,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretVisitor {
            type Value = ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_client_id: Option<String> = None;
                let mut azure_client_secret_key: Option<String> = None;
                let mut azure_credential_kind: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretKind> = None;
                let mut azure_tenant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_client_id" => {
                            azure_client_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_client_secret_key" => {
                            azure_client_secret_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_credential_kind" => {
                            azure_credential_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _azure_credential_kind) = azure_credential_kind {
                                match _azure_credential_kind {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretKind::UnparsedObject(_azure_credential_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "azure_tenant_id" => {
                            azure_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let azure_client_id =
                    azure_client_id.ok_or_else(|| M::Error::missing_field("azure_client_id"))?;
                let azure_client_secret_key = azure_client_secret_key
                    .ok_or_else(|| M::Error::missing_field("azure_client_secret_key"))?;
                let azure_credential_kind = azure_credential_kind
                    .ok_or_else(|| M::Error::missing_field("azure_credential_kind"))?;
                let azure_tenant_id =
                    azure_tenant_id.ok_or_else(|| M::Error::missing_field("azure_tenant_id"))?;

                let content = ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret {
                    azure_client_id,
                    azure_client_secret_key,
                    azure_credential_kind,
                    azure_tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecretVisitor,
        )
    }
}
