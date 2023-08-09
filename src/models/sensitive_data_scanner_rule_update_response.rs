/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerRuleUpdateResponse : Update rule response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleUpdateResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::SensitiveDataScannerMetaVersionOnly>>,
}

impl SensitiveDataScannerRuleUpdateResponse {
    /// Update rule response.
    pub fn new() -> SensitiveDataScannerRuleUpdateResponse {
        SensitiveDataScannerRuleUpdateResponse {
            meta: None,
        }
    }
}


