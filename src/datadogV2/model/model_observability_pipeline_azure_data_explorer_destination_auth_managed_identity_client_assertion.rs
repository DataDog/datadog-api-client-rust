// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authenticate using a managed identity as a client assertion for a Microsoft Entra application.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion {
    /// The Azure credential kind. The value should always be `managed_identity_client_assertion`.
    #[serde(rename = "azure_credential_kind")]
    pub azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionKind,
    /// The client ID of the Microsoft Entra application that trusts the managed identity.
    #[serde(rename = "client_assertion_client_id")]
    pub client_assertion_client_id: String,
    /// The tenant ID of the Microsoft Entra application that trusts the managed identity.
    #[serde(rename = "client_assertion_tenant_id")]
    pub client_assertion_tenant_id: String,
    /// The ID of the user-assigned managed identity. If omitted, the system-assigned managed identity is used.
    #[serde(rename = "user_assigned_managed_identity_id", default, with = "::serde_with::rust::double_option")]
    pub user_assigned_managed_identity_id: Option<Option<String>>,
    /// The type of the user-assigned managed identity ID.
    #[serde(rename = "user_assigned_managed_identity_id_type")]
    pub user_assigned_managed_identity_id_type: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion {
    pub fn new(
        azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionKind,
        client_assertion_client_id: String,
        client_assertion_tenant_id: String,
    ) -> ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion {
        ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion {
            azure_credential_kind,
            client_assertion_client_id,
            client_assertion_tenant_id,
            user_assigned_managed_identity_id: None,
            user_assigned_managed_identity_id_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn user_assigned_managed_identity_id(mut self, value: Option<String>) -> Self {
        self.user_assigned_managed_identity_id = Some(value);
        self
    }

    pub fn user_assigned_managed_identity_id_type(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType,
    ) -> Self {
        self.user_assigned_managed_identity_id_type = Some(value);
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
    for ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionVisitor {
            type Value = ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_credential_kind: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionKind> = None;
                let mut client_assertion_client_id: Option<String> = None;
                let mut client_assertion_tenant_id: Option<String> = None;
                let mut user_assigned_managed_identity_id: Option<Option<String>> = None;
                let mut user_assigned_managed_identity_id_type: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_credential_kind" => {
                            azure_credential_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _azure_credential_kind) = azure_credential_kind {
                                match _azure_credential_kind {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionKind::UnparsedObject(_azure_credential_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "client_assertion_client_id" => {
                            client_assertion_client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "client_assertion_tenant_id" => {
                            client_assertion_tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "user_assigned_managed_identity_id" => {
                            user_assigned_managed_identity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "user_assigned_managed_identity_id_type" => {
                            if v.is_null() {
                                continue;
                            }
                            user_assigned_managed_identity_id_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _user_assigned_managed_identity_id_type) = user_assigned_managed_identity_id_type {
                                match _user_assigned_managed_identity_id_type {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType::UnparsedObject(_user_assigned_managed_identity_id_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let azure_credential_kind = azure_credential_kind.ok_or_else(|| M::Error::missing_field("azure_credential_kind"))?;
                let client_assertion_client_id = client_assertion_client_id.ok_or_else(|| M::Error::missing_field("client_assertion_client_id"))?;
                let client_assertion_tenant_id = client_assertion_tenant_id.ok_or_else(|| M::Error::missing_field("client_assertion_tenant_id"))?;

                let content = ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion {
                    azure_credential_kind,
                    client_assertion_client_id,
                    client_assertion_tenant_id,
                    user_assigned_managed_identity_id,
                    user_assigned_managed_identity_id_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertionVisitor)
    }
}
