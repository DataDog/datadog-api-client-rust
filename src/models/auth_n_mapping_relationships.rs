/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AuthNMappingRelationships : All relationships associated with AuthN Mapping.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthNMappingRelationships {
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::RelationshipToRole>>,
    #[serde(rename = "saml_assertion_attribute", skip_serializing_if = "Option::is_none")]
    pub saml_assertion_attribute: Option<Box<crate::models::RelationshipToSamlAssertionAttribute>>,
}

impl AuthNMappingRelationships {
    /// All relationships associated with AuthN Mapping.
    pub fn new() -> AuthNMappingRelationships {
        AuthNMappingRelationships {
            role: None,
            saml_assertion_attribute: None,
        }
    }
}


