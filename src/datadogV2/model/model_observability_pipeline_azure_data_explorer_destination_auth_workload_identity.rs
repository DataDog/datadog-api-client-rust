// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authenticate using Azure Workload Identity (for example, on Kubernetes).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity {
    /// The Azure credential kind. The value should always be `workload_identity`.
    #[serde(rename = "azure_credential_kind")]
    pub azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityKind,
    /// The client ID of the Microsoft Entra application. If omitted, it is read from the environment.
    #[serde(rename = "client_id", default, with = "::serde_with::rust::double_option")]
    pub client_id: Option<Option<String>>,
    /// The Microsoft Entra tenant ID. If omitted, it is read from the environment.
    #[serde(rename = "tenant_id", default, with = "::serde_with::rust::double_option")]
    pub tenant_id: Option<Option<String>>,
    /// Path to the federated token file. If omitted, it is read from the environment.
    #[serde(rename = "token_file_path", default, with = "::serde_with::rust::double_option")]
    pub token_file_path: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity {
    pub fn new(
        azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityKind,
    ) -> ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity {
        ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity {
            azure_credential_kind,
            client_id: None,
            tenant_id: None,
            token_file_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn client_id(mut self, value: Option<String>) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: Option<String>) -> Self {
        self.tenant_id = Some(value);
        self
    }

    pub fn token_file_path(mut self, value: Option<String>) -> Self {
        self.token_file_path = Some(value);
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

impl<'de> Deserialize<'de>
    for ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityVisitor;
        impl<'a> Visitor<'a>
            for ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityVisitor
        {
            type Value = ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_credential_kind: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityKind> = None;
                let mut client_id: Option<Option<String>> = None;
                let mut tenant_id: Option<Option<String>> = None;
                let mut token_file_path: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_credential_kind" => {
                            azure_credential_kind =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _azure_credential_kind) = azure_credential_kind {
                                match _azure_credential_kind {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityKind::UnparsedObject(_azure_credential_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_file_path" => {
                            token_file_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let azure_credential_kind = azure_credential_kind
                    .ok_or_else(|| M::Error::missing_field("azure_credential_kind"))?;

                let content =
                    ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity {
                        azure_credential_kind,
                        client_id,
                        tenant_id,
                        token_file_path,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentityVisitor,
        )
    }
}
