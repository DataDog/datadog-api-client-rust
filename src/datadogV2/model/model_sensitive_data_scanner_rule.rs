// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Rule item included in the group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRule {
    /// ID of the rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sensitive Data Scanner rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerRuleType>,
}

impl SensitiveDataScannerRule {
    pub fn new() -> SensitiveDataScannerRule {
        SensitiveDataScannerRule {
            id: None,
            type_: None,
        }
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for SensitiveDataScannerRule {
    fn default() -> Self {
        Self::new()
    }
}
