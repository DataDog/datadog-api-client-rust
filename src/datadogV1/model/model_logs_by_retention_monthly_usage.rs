// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing a summary of indexed logs usage by retention period for a single month.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsByRetentionMonthlyUsage {
    /// The month for the usage.
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// Indexed logs usage for each active retention for the month.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::LogsRetentionSumUsage>>,
}

impl LogsByRetentionMonthlyUsage {
    pub fn new() -> LogsByRetentionMonthlyUsage {
        LogsByRetentionMonthlyUsage {
            date: None,
            usage: None,
        }
    }
}