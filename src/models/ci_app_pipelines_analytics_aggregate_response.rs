/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CiAppPipelinesAnalyticsAggregateResponse : The response object for the pipeline events aggregate API endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CiAppPipelinesAnalyticsAggregateResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::CiAppPipelinesAggregationBucketsResponse>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::CiAppResponseLinks>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::CiAppResponseMetadata>>,
}

impl CiAppPipelinesAnalyticsAggregateResponse {
    /// The response object for the pipeline events aggregate API endpoint.
    pub fn new() -> CiAppPipelinesAnalyticsAggregateResponse {
        CiAppPipelinesAnalyticsAggregateResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}


