// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A standard pattern.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerStandardPatternData {
    /// Data containing the standard pattern id.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::SensitiveDataScannerStandardPattern>>,
}

impl SensitiveDataScannerStandardPatternData {
    pub fn new() -> SensitiveDataScannerStandardPatternData {
        SensitiveDataScannerStandardPatternData { data: None }
    }
}
