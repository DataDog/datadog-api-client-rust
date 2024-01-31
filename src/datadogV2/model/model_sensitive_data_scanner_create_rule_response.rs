// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create rule response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerCreateRuleResponse {
    /// Response data related to the creation of a rule.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerRuleResponse>,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerCreateRuleResponse {
    pub fn new() -> SensitiveDataScannerCreateRuleResponse {
        SensitiveDataScannerCreateRuleResponse {
            data: None,
            meta: None,
        }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleResponse,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}
impl Default for SensitiveDataScannerCreateRuleResponse {
    fn default() -> Self {
        Self::new()
    }
}
