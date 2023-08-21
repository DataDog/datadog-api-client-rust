// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1 {
    /// Contact information about the service.
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: ServiceDefinitionV1Contact,
    /// Extensions to V1 schema.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: map[string]interface{},
    /// A list of external links related to the services.
    #[serde(rename = "external-resources", skip_serializing_if = "Option::is_none")]
    pub external_resources: Vec<ServiceDefinitionV1Resource>,
    /// Basic information about a service.
    #[serde(rename = "info")]
    pub info: ServiceDefinitionV1Info,
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: ServiceDefinitionV1Integrations,
    /// Org related information about the service.
    #[serde(rename = "org", skip_serializing_if = "Option::is_none")]
    pub org: ServiceDefinitionV1Org,
    /// Schema version being used.
    #[serde(rename = "schema-version", skip_serializing_if = "Option::is_none")]
    pub schema_version: ServiceDefinitionV1Version,
    /// A set of custom tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
}

