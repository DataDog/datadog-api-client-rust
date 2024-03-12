// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a Datadog Log index.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndex {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit")]
    pub daily_limit: Option<i64>,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
    /// following the order of the array. Only the first matching active exclusion matters,
    /// others (if any) are ignored.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>>,
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV1::model::LogsFilter,
    /// A boolean stating if the index is rate limited, meaning more logs than the daily limit have been sent.
    /// Rate limit is reset every-day at 2pm UTC.
    #[serde(rename = "is_rate_limited")]
    pub is_rate_limited: Option<bool>,
    /// The name of the index.
    #[serde(rename = "name")]
    pub name: String,
    /// The number of days before logs are deleted from this index. Available values depend on
    /// retention plans specified in your organization's contract/subscriptions.
    #[serde(rename = "num_retention_days")]
    pub num_retention_days: Option<i64>,
}

impl LogsIndex {
    pub fn new(filter: crate::datadogV1::model::LogsFilter, name: String) -> LogsIndex {
        LogsIndex {
            daily_limit: None,
            exclusion_filters: None,
            filter,
            is_rate_limited: None,
            name,
            num_retention_days: None,
        }
    }

    pub fn daily_limit(mut self, value: i64) -> Self {
        self.daily_limit = Some(value);
        self
    }

    pub fn exclusion_filters(mut self, value: Vec<crate::datadogV1::model::LogsExclusion>) -> Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn is_rate_limited(mut self, value: bool) -> Self {
        self.is_rate_limited = Some(value);
        self
    }

    pub fn num_retention_days(mut self, value: i64) -> Self {
        self.num_retention_days = Some(value);
        self
    }
}
