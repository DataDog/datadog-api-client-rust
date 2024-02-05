// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SensitiveDataScannerRuleType {
    #[serde(rename = "sensitive_data_scanner_rule")]
    SENSITIVE_DATA_SCANNER_RULE,
}

impl ToString for SensitiveDataScannerRuleType {
    fn to_string(&self) -> String {
        match self {
            Self::SENSITIVE_DATA_SCANNER_RULE => String::from("sensitive_data_scanner_rule"),
        }
    }
}
