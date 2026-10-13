// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authenticate using an Azure managed identity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity {
    /// The Azure credential kind. The value should always be `managed_identity`.
    #[serde(rename = "azure_credential_kind")]
    pub azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityKind,
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

impl ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity {
    pub fn new(
        azure_credential_kind: crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityKind,
    ) -> ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity {
        ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity {
            azure_credential_kind,
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
    for ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityVisitor;
        impl<'a> Visitor<'a>
            for ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityVisitor
        {
            type Value = ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_credential_kind: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityKind> = None;
                let mut user_assigned_managed_identity_id: Option<Option<String>> = None;
                let mut user_assigned_managed_identity_id_type: Option<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType> = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityKind::UnparsedObject(_azure_credential_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "user_assigned_managed_identity_id" => {
                            user_assigned_managed_identity_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_assigned_managed_identity_id_type" => {
                            if v.is_null() {
                                continue;
                            }
                            user_assigned_managed_identity_id_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _user_assigned_managed_identity_id_type) =
                                user_assigned_managed_identity_id_type
                            {
                                match _user_assigned_managed_identity_id_type {
                                    crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationManagedIdentityIdType::UnparsedObject(_user_assigned_managed_identity_id_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                    ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity {
                        azure_credential_kind,
                        user_assigned_managed_identity_id,
                        user_assigned_managed_identity_id_type,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityVisitor,
        )
    }
}
