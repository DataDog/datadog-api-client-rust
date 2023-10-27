// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditLogsWarning {
    /// Unique code for this type of warning.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Detailed explanation of this specific warning.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// Short human-readable summary of the warning.
    #[serde(rename = "title")]
    pub title: Option<String>,
}

impl AuditLogsWarning {
    /// Warning message indicating something that went wrong with the query.
    pub fn new() -> AuditLogsWarning {
        AuditLogsWarning {
            code: None,
            detail: None,
            title: None,
        }
    }
}
