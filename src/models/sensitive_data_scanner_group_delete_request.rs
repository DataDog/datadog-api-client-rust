/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerGroupDeleteRequest : Delete group request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupDeleteRequest {
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerGroupDeleteRequest {
    /// Delete group request.
    pub fn new(meta: crate::models::SensitiveDataScannerMetaVersionOnly) -> SensitiveDataScannerGroupDeleteRequest {
        SensitiveDataScannerGroupDeleteRequest {
            meta: Box::new(meta),
        }
    }
}


