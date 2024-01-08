// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupRelationships {
    /// A Sensitive Data Scanner configuration data.
    #[serde(rename = "configuration")]
    pub configuration: Option<Box<crate::datadogV2::model::SensitiveDataScannerConfigurationData>>,
    /// Rules included in the group.
    #[serde(rename = "rules")]
    pub rules: Option<Box<crate::datadogV2::model::SensitiveDataScannerRuleData>>,
}

impl SensitiveDataScannerGroupRelationships {
    pub fn new() -> SensitiveDataScannerGroupRelationships {
        SensitiveDataScannerGroupRelationships {
            configuration: None,
            rules: None,
        }
    }
}
impl Default for SensitiveDataScannerGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}
