// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the number of logs for each hour.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsResponse {
    /// An array of objects regarding hourly usage of logs.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageLogsHour>>,
}

impl UsageLogsResponse {
    pub fn new() -> UsageLogsResponse {
        UsageLogsResponse { usage: None }
    }
}
