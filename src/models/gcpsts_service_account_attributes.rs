/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// GcpstsServiceAccountAttributes : Attributes associated with your service account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GcpstsServiceAccountAttributes {
    /// Silence monitors for expected GCE instance shutdowns.
    #[serde(rename = "automute", skip_serializing_if = "Option::is_none")]
    pub automute: Option<bool>,
    /// Your service account email address.
    #[serde(rename = "client_email", skip_serializing_if = "Option::is_none")]
    pub client_email: Option<String>,
    /// Your Host Filters.
    #[serde(rename = "host_filters", skip_serializing_if = "Option::is_none")]
    pub host_filters: Option<Vec<String>>,
    /// When enabled, Datadog performs configuration checks across your Google Cloud environment by continuously scanning every resource.
    #[serde(rename = "is_cspm_enabled", skip_serializing_if = "Option::is_none")]
    pub is_cspm_enabled: Option<bool>,
}

impl GcpstsServiceAccountAttributes {
    /// Attributes associated with your service account.
    pub fn new() -> GcpstsServiceAccountAttributes {
        GcpstsServiceAccountAttributes {
            automute: None,
            client_email: None,
            host_filters: None,
            is_cspm_enabled: None,
        }
    }
}


