// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Rules included in the group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleData {
    /// Rules included in the group. The order is important.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::SensitiveDataScannerRule>>,
}

impl SensitiveDataScannerRuleData {
    pub fn new() -> SensitiveDataScannerRuleData {
        SensitiveDataScannerRuleData { data: None }
    }
}
