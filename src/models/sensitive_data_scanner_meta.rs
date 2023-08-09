/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SensitiveDataScannerMeta : Meta response containing information about the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SensitiveDataScannerMeta {
    /// Maximum number of scanning rules allowed for the org.
    #[serde(rename = "count_limit", skip_serializing_if = "Option::is_none")]
    pub count_limit: Option<i64>,
    /// Maximum number of scanning groups allowed for the org.
    #[serde(rename = "group_count_limit", skip_serializing_if = "Option::is_none")]
    pub group_count_limit: Option<i64>,
    /// Whether or not scanned events are highlighted in Logs or RUM for the org.
    #[serde(rename = "has_highlight_enabled", skip_serializing_if = "Option::is_none")]
    pub has_highlight_enabled: Option<bool>,
    /// Whether or not scanned events have multi-pass enabled.
    #[serde(rename = "has_multi_pass_enabled", skip_serializing_if = "Option::is_none")]
    pub has_multi_pass_enabled: Option<bool>,
    /// Whether or not the org is compliant to the payment card industry standard.
    #[serde(rename = "is_pci_compliant", skip_serializing_if = "Option::is_none")]
    pub is_pci_compliant: Option<bool>,
    /// Version of the API.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl SensitiveDataScannerMeta {
    /// Meta response containing information about the API.
    pub fn new() -> SensitiveDataScannerMeta {
        SensitiveDataScannerMeta {
            count_limit: None,
            group_count_limit: None,
            has_highlight_enabled: None,
            has_multi_pass_enabled: None,
            is_pci_compliant: None,
            version: None,
        }
    }
}


