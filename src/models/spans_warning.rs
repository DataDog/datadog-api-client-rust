/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansWarning : A warning message indicating something that went wrong with the query.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansWarning {
    /// A unique code for this type of warning.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A detailed explanation of this specific warning.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A short human-readable summary of the warning.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl SpansWarning {
    /// A warning message indicating something that went wrong with the query.
    pub fn new() -> SpansWarning {
        SpansWarning {
            code: None,
            detail: None,
            title: None,
        }
    }
}


