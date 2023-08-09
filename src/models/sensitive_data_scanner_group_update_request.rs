/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerGroupUpdateRequest : Update group request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupUpdateRequest {
    #[serde(rename = "data")]
    pub data: Box<crate::models::SensitiveDataScannerGroupUpdate>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerGroupUpdateRequest {
    /// Update group request.
    pub fn new(data: crate::models::SensitiveDataScannerGroupUpdate, meta: crate::models::SensitiveDataScannerMetaVersionOnly) -> SensitiveDataScannerGroupUpdateRequest {
        SensitiveDataScannerGroupUpdateRequest {
            data: Box::new(data),
            meta: Box::new(meta),
        }
    }
}


