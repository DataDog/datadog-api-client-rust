// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Info {
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service", skip_serializing_if = "Option::is_none")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// A friendly name of the service.
    #[serde(rename = "display-name", skip_serializing_if = "Option::is_none")]
    pub display_name: String,
    /// Service tier.
    #[serde(rename = "service-tier", skip_serializing_if = "Option::is_none")]
    pub service_tier: String,
}

