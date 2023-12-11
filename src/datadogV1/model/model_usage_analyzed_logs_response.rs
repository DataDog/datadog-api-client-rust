// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A response containing the number of analyzed logs for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageAnalyzedLogsResponse {
    /// Get hourly usage for analyzed logs.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageAnalyzedLogsHour>>,
}

impl UsageAnalyzedLogsResponse {
    pub fn new() -> UsageAnalyzedLogsResponse {
        UsageAnalyzedLogsResponse { usage: None }
    }
}
