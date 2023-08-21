// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Dot1 {
    /// Identifier for a group of related services serving a product feature, which the service is a part of.
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: String,
    /// A list of contacts related to the services.
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Vec<ServiceDefinitionV2Dot1Contact>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service", skip_serializing_if = "Option::is_none")]
    pub dd_service: String,
    /// A short description of the service.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Extensions to v2.1 schema.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: map[string]interface{},
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: ServiceDefinitionV2Dot1Integrations,
    /// The current life cycle phase of the service.
    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: String,
    /// A list of links related to the services.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Vec<ServiceDefinitionV2Dot1Link>,
    /// Schema version being used.
    #[serde(rename = "schema-version", skip_serializing_if = "Option::is_none")]
    pub schema_version: ServiceDefinitionV2Dot1Version,
    /// A set of custom tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Team that owns the service. It is used to locate a team defined in Datadog Teams if it exists.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: String,
    /// Importance of the service.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: String,
}

