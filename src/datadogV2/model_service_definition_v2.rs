// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2 {
    /// A list of contacts related to the services.
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Vec<ServiceDefinitionV2Contact>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service", skip_serializing_if = "Option::is_none")]
    pub dd_service: String,
    /// Experimental feature. A Team handle that matches a Team in the Datadog Teams product.
    #[serde(rename = "dd-team", skip_serializing_if = "Option::is_none")]
    pub dd_team: String,
    /// A list of documentation related to the services.
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none")]
    pub docs: Vec<ServiceDefinitionV2Doc>,
    /// Extensions to V2 schema.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: map[string]interface{},
    /// Third party integrations that Datadog supports.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: ServiceDefinitionV2Integrations,
    /// A list of links related to the services.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Vec<ServiceDefinitionV2Link>,
    /// A list of code repositories related to the services.
    #[serde(rename = "repos", skip_serializing_if = "Option::is_none")]
    pub repos: Vec<ServiceDefinitionV2Repo>,
    /// Schema version being used.
    #[serde(rename = "schema-version", skip_serializing_if = "Option::is_none")]
    pub schema_version: ServiceDefinitionV2Version,
    /// A set of custom tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Team that owns the service.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: String,
}

