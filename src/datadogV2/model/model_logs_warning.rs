// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A warning message indicating something that went wrong with the query
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsWarning {
    /// A unique code for this type of warning
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// A detailed explanation of this specific warning
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// A short human-readable summary of the warning
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl LogsWarning {
    pub fn new() -> LogsWarning {
        LogsWarning {
            code: None,
            detail: None,
            title: None,
        }
    }
}
impl Default for LogsWarning {
    fn default() -> Self {
        Self::new()
    }
}
