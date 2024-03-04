// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Optional metadata associated to the response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterMeta {
    /// A warning message.
    #[serde(rename = "warning")]
    pub warning: Option<String>,
}

impl SecurityFilterMeta {
    pub fn new() -> SecurityFilterMeta {
        SecurityFilterMeta { warning: None }
    }

    pub fn warning(mut self, value: String) -> Self {
        self.warning = Some(value);
        self
    }
}

impl Default for SecurityFilterMeta {
    fn default() -> Self {
        Self::new()
    }
}
