// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication configuration for Azure Data Explorer. The `azure_credential_kind` field selects the credential type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineAzureDataExplorerDestinationAuth {
    ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli>),
	ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret>),
	ObservabilityPipelineAzureDataExplorerDestinationAuthClientCertificate(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientCertificate>),
	ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity>),
	ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion>),
	ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity(Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAzureDataExplorerDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthAzureCli(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthClientSecret(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthClientCertificate>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthClientCertificate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentity(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthManagedIdentityClientAssertion(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureDataExplorerDestinationAuth::ObservabilityPipelineAzureDataExplorerDestinationAuthWorkloadIdentity(_v));
            }
        }

        return Ok(
            ObservabilityPipelineAzureDataExplorerDestinationAuth::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
