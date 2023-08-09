/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceDefinitionDataAttributes : Service definition attributes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceDefinitionDataAttributes {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ServiceDefinitionMeta>>,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<crate::models::ServiceDefinitionSchema>>,
}

impl ServiceDefinitionDataAttributes {
    /// Service definition attributes.
    pub fn new() -> ServiceDefinitionDataAttributes {
        ServiceDefinitionDataAttributes {
            meta: None,
            schema: None,
        }
    }
}


