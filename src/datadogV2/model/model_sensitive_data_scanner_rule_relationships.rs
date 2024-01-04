// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of a scanning rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleRelationships {
    /// A scanning group data.
    #[serde(rename = "group")]
    pub group: Option<Box<crate::datadogV2::model::SensitiveDataScannerGroupData>>,
    /// A standard pattern.
    #[serde(rename = "standard_pattern")]
    pub standard_pattern:
        Option<Box<crate::datadogV2::model::SensitiveDataScannerStandardPatternData>>,
}

impl SensitiveDataScannerRuleRelationships {
    pub fn new() -> SensitiveDataScannerRuleRelationships {
        SensitiveDataScannerRuleRelationships {
            group: None,
            standard_pattern: None,
        }
    }
}
