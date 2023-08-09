/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentSearchResponse : Response with incidents and facets.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentSearchResponse {
    #[serde(rename = "data")]
    pub data: Box<crate::models::IncidentSearchResponseData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<crate::models::IncidentResponseIncludedItem>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::IncidentSearchResponseMeta>>,
}

impl IncidentSearchResponse {
    /// Response with incidents and facets.
    pub fn new(data: crate::models::IncidentSearchResponseData) -> IncidentSearchResponse {
        IncidentSearchResponse {
            data: Box::new(data),
            included: None,
            meta: None,
        }
    }
}


