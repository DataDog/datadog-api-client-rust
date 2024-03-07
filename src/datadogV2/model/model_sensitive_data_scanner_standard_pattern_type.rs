// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SensitiveDataScannerStandardPatternType {
    #[serde(rename = "sensitive_data_scanner_standard_pattern")]
    SENSITIVE_DATA_SCANNER_STANDARD_PATTERN,
}

impl ToString for SensitiveDataScannerStandardPatternType {
    fn to_string(&self) -> String {
        match self {
            Self::SENSITIVE_DATA_SCANNER_STANDARD_PATTERN => {
                String::from("sensitive_data_scanner_standard_pattern")
            }
        }
    }
}
