// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A scanning group data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupData {
    /// A scanning group.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerGroup>,
}

impl SensitiveDataScannerGroupData {
    pub fn new() -> SensitiveDataScannerGroupData {
        SensitiveDataScannerGroupData { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::SensitiveDataScannerGroup) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupData {
    fn default() -> Self {
        Self::new()
    }
}
