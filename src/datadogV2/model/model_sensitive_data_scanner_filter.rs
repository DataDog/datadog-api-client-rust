// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Filter for the Scanning Group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerFilter {
    /// Query to filter the events.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SensitiveDataScannerFilter {
    pub fn new() -> SensitiveDataScannerFilter {
        SensitiveDataScannerFilter { query: None }
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerFilter {
    fn default() -> Self {
        Self::new()
    }
}
