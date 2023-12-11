// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The number of analyzed logs for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageAnalyzedLogsHour {
    /// Contains the number of analyzed logs.
    #[serde(
        rename = "analyzed_logs",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub analyzed_logs: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageAnalyzedLogsHour {
    pub fn new() -> UsageAnalyzedLogsHour {
        UsageAnalyzedLogsHour {
            analyzed_logs: None,
            hour: None,
            org_name: None,
            public_id: None,
        }
    }
}
