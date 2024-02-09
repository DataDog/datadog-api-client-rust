// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper object with the list of monitor IDs.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteMonitorResponseData {
    /// An array of of Monitor IDs that can be safely deleted.
    #[serde(rename = "ok")]
    pub ok: Option<Vec<i64>>,
}

impl CheckCanDeleteMonitorResponseData {
    pub fn new() -> CheckCanDeleteMonitorResponseData {
        CheckCanDeleteMonitorResponseData { ok: None }
    }

    pub fn ok(&mut self, value: Vec<i64>) -> &mut Self {
        self.ok = Some(value);
        self
    }
}

impl Default for CheckCanDeleteMonitorResponseData {
    fn default() -> Self {
        Self::new()
    }
}
