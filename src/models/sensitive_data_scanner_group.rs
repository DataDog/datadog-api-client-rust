/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerGroup : A scanning group.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroup {
    /// ID of the group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroup {
    /// A scanning group.
    pub fn new() -> SensitiveDataScannerGroup {
        SensitiveDataScannerGroup {
            id: None,
            r#type: None,
        }
    }
}


