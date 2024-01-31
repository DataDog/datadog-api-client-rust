// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Meta payload containing information about the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerMetaVersionOnly {
    /// Version of the API (optional).
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SensitiveDataScannerMetaVersionOnly {
    pub fn new() -> SensitiveDataScannerMetaVersionOnly {
        SensitiveDataScannerMetaVersionOnly { version: None }
    }

    pub fn with_version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}
impl Default for SensitiveDataScannerMetaVersionOnly {
    fn default() -> Self {
        Self::new()
    }
}
