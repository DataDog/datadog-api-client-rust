/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FastlyAccountCreateRequestAttributes : Attributes object for creating a Fastly account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FastlyAccountCreateRequestAttributes {
    /// The API key for the Fastly account.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The name of the Fastly account.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of services belonging to the parent account.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::FastlyService>>,
}

impl FastlyAccountCreateRequestAttributes {
    /// Attributes object for creating a Fastly account.
    pub fn new(api_key: String, name: String) -> FastlyAccountCreateRequestAttributes {
        FastlyAccountCreateRequestAttributes {
            api_key,
            name,
            services: None,
        }
    }
}


