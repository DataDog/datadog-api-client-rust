// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the Sensitive Data Scanner group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupAttributes {
    /// Description of the group.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Filter for the Scanning Group.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::SensitiveDataScannerFilter>>,
    /// Whether or not the group is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of products the scanning group applies.
    #[serde(rename = "product_list")]
    pub product_list: Option<Vec<crate::datadogV2::model::SensitiveDataScannerProduct>>,
}

impl SensitiveDataScannerGroupAttributes {
    pub fn new() -> SensitiveDataScannerGroupAttributes {
        SensitiveDataScannerGroupAttributes {
            description: None,
            filter: None,
            is_enabled: None,
            name: None,
            product_list: None,
        }
    }
}
