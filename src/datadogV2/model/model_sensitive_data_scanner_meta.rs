// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Meta response containing information about the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerMeta {
    /// Maximum number of scanning rules allowed for the org.
    #[serde(rename = "count_limit")]
    pub count_limit: Option<i64>,
    /// Maximum number of scanning groups allowed for the org.
    #[serde(rename = "group_count_limit")]
    pub group_count_limit: Option<i64>,
    /// Whether or not scanned events are highlighted in Logs or RUM for the org.
    #[serde(rename = "has_highlight_enabled")]
    pub has_highlight_enabled: Option<bool>,
    /// Whether or not scanned events have multi-pass enabled.
    #[serde(rename = "has_multi_pass_enabled")]
    pub has_multi_pass_enabled: Option<bool>,
    /// Whether or not the org is compliant to the payment card industry standard.
    #[serde(rename = "is_pci_compliant")]
    pub is_pci_compliant: Option<bool>,
    /// Version of the API.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SensitiveDataScannerMeta {
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