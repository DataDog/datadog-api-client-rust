// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An array of service level objective objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteSLOResponseData {
    /// An array of of SLO IDs that can be safely deleted.
    #[serde(rename = "ok")]
    pub ok: Option<Vec<String>>,
}

impl CheckCanDeleteSLOResponseData {
    pub fn new() -> CheckCanDeleteSLOResponseData {
        CheckCanDeleteSLOResponseData { ok: None }
    }
}
impl Default for CheckCanDeleteSLOResponseData {
    fn default() -> Self {
        Self::new()
    }
}
