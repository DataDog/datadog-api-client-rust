// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Standard pattern item.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerStandardPatternsResponseItem {
    /// Attributes of the Sensitive Data Scanner standard pattern.
    #[serde(rename = "attributes")]
    pub attributes:
        Option<Box<crate::datadogV2::model::SensitiveDataScannerStandardPatternAttributes>>,
    /// ID of the standard pattern.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sensitive Data Scanner standard pattern type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerStandardPatternType>,
}

impl SensitiveDataScannerStandardPatternsResponseItem {
    pub fn new() -> SensitiveDataScannerStandardPatternsResponseItem {
        SensitiveDataScannerStandardPatternsResponseItem {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for SensitiveDataScannerStandardPatternsResponseItem {
    fn default() -> Self {
        Self::new()
    }
}
