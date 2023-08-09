/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// PartialApplicationKeyAttributes : Attributes of a partial application key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartialApplicationKeyAttributes {
    /// Creation date of the application key.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The last four characters of the application key.
    #[serde(rename = "last4", skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Name of the application key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of scopes to grant the application key.
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Option<Vec<String>>>,
}

impl PartialApplicationKeyAttributes {
    /// Attributes of a partial application key.
    pub fn new() -> PartialApplicationKeyAttributes {
        PartialApplicationKeyAttributes {
            created_at: None,
            last4: None,
            name: None,
            scopes: None,
        }
    }
}


