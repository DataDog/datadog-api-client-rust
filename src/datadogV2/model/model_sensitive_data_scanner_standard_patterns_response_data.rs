// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List Standard patterns response data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerStandardPatternsResponseData {
    /// List Standard patterns response.
    #[serde(rename = "data")]
    pub data:
        Option<Vec<crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseItem>>,
}

impl SensitiveDataScannerStandardPatternsResponseData {
    pub fn new() -> SensitiveDataScannerStandardPatternsResponseData {
        SensitiveDataScannerStandardPatternsResponseData { data: None }
    }
}