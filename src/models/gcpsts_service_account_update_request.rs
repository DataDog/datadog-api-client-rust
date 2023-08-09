/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// GcpstsServiceAccountUpdateRequest : Service account info.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GcpstsServiceAccountUpdateRequest {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::GcpstsServiceAccountUpdateRequestData>>,
}

impl GcpstsServiceAccountUpdateRequest {
    /// Service account info.
    pub fn new() -> GcpstsServiceAccountUpdateRequest {
        GcpstsServiceAccountUpdateRequest {
            data: None,
        }
    }
}


