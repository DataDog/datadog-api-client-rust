/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceDefinitionV2 : Service definition V2 for providing service metadata and integrations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceDefinitionV2 {
    /// A list of contacts related to the services.
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<crate::models::ServiceDefinitionV2Contact>>,
    /// Unique identifier of the service. Must be unique across all services and is used to match with a service in Datadog.
    #[serde(rename = "dd-service")]
    pub dd_service: String,
    /// Experimental feature. A Team handle that matches a Team in the Datadog Teams product.
    #[serde(rename = "dd-team", skip_serializing_if = "Option::is_none")]
    pub dd_team: Option<String>,
    /// A list of documentation related to the services.
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<crate::models::ServiceDefinitionV2Doc>>,
    /// Extensions to V2 schema.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Box<crate::models::ServiceDefinitionV2Integrations>>,
    /// A list of links related to the services.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::ServiceDefinitionV2Link>>,
    /// A list of code repositories related to the services.
    #[serde(rename = "repos", skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<crate::models::ServiceDefinitionV2Repo>>,
    #[serde(rename = "schema-version")]
    pub schema_version: crate::models::ServiceDefinitionV2Version,
    /// A set of custom tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Team that owns the service.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

impl ServiceDefinitionV2 {
    /// Service definition V2 for providing service metadata and integrations.
    pub fn new(dd_service: String, schema_version: crate::models::ServiceDefinitionV2Version) -> ServiceDefinitionV2 {
        ServiceDefinitionV2 {
            contacts: None,
            dd_service,
            dd_team: None,
            docs: None,
            extensions: None,
            integrations: None,
            links: None,
            repos: None,
            schema_version,
            tags: None,
            team: None,
        }
    }
}


