// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentIntegrationMetadataType {
    #[serde(rename = "incident_integrations")]
	INCIDENT_INTEGRATIONS,
}

impl ToString for IncidentIntegrationMetadataType {
    fn to_string(&self) -> String {
        match self {
            Self::INCIDENT_INTEGRATIONS => String::from("incident_integrations"),
        }
    }
}

impl Default for IncidentIntegrationMetadataType {
    fn default() -> IncidentIntegrationMetadataType {
        Self::INCIDENT_INTEGRATIONS
    }
}
