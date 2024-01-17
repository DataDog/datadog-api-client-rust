// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Audit logs usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAuditLogsHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The total number of audit logs lines indexed during a given hour.
    #[serde(
        rename = "lines_indexed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub lines_indexed: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageAuditLogsHour {
    pub fn new() -> UsageAuditLogsHour {
        UsageAuditLogsHour {
            hour: None,
            lines_indexed: None,
            org_name: None,
            public_id: None,
        }
    }
}
impl Default for UsageAuditLogsHour {
    fn default() -> Self {
        Self::new()
    }
}
