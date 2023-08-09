/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentServicesResponse : Response with a list of incident service payloads.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentServicesResponse {
    /// An array of incident services.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::IncidentServiceResponseData>,
    /// Included related resources which the user requested.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::IncidentServiceIncludedItems>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::IncidentResponseMeta>>,
}

impl IncidentServicesResponse {
    /// Response with a list of incident service payloads.
    pub fn new(data: Vec<crate::models::IncidentServiceResponseData>) -> IncidentServicesResponse {
        IncidentServicesResponse {
            data,
            included: None,
            meta: None,
        }
    }
}


