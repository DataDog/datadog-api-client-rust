/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerGroupIncludedItem : A Scanning Group included item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupIncludedItem {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::SensitiveDataScannerGroupAttributes>>,
    /// ID of the group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "relationships", skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Box<crate::models::SensitiveDataScannerGroupRelationships>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroupIncludedItem {
    /// A Scanning Group included item.
    pub fn new() -> SensitiveDataScannerGroupIncludedItem {
        SensitiveDataScannerGroupIncludedItem {
            attributes: None,
            id: None,
            relationships: None,
            r#type: None,
        }
    }
}


