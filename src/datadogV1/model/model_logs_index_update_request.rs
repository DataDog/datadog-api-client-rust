// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for updating a Datadog Log index.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexUpdateRequest {
    /// The number of log events you can send in this index per day before you are rate-limited.
    #[serde(rename = "daily_limit")]
    pub daily_limit: Option<i64>,
    /// If true, sets the `daily_limit` value to null and the index is not limited on a daily basis (any
    /// specified `daily_limit` value in the request is ignored). If false or omitted, the index's current
    /// `daily_limit` is maintained.
    #[serde(rename = "disable_daily_limit")]
    pub disable_daily_limit: Option<bool>,
    /// An array of exclusion objects. The logs are tested against the query of each filter,
    /// following the order of the array. Only the first matching active exclusion matters,
    /// others (if any) are ignored.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV1::model::LogsExclusion>>,
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV1::model::LogsFilter,
    /// The number of days before logs are deleted from this index. Available values depend on
    /// retention plans specified in your organization's contract/subscriptions.
    ///
    /// **Note:** Changing the retention for an index adjusts the length of retention for all logs
    /// already in this index. It may also affect billing.
    #[serde(rename = "num_retention_days")]
    pub num_retention_days: Option<i64>,
}

impl LogsIndexUpdateRequest {
    pub fn new(filter: crate::datadogV1::model::LogsFilter) -> LogsIndexUpdateRequest {
        LogsIndexUpdateRequest {
            daily_limit: None,
            disable_daily_limit: None,
            exclusion_filters: None,
            filter,
            num_retention_days: None,
        }
    }

    pub fn with_daily_limit(&mut self, value: i64) -> &mut Self {
        self.daily_limit = Some(value);
        self
    }

    pub fn with_disable_daily_limit(&mut self, value: bool) -> &mut Self {
        self.disable_daily_limit = Some(value);
        self
    }

    pub fn with_exclusion_filters(
        &mut self,
        value: Vec<crate::datadogV1::model::LogsExclusion>,
    ) -> &mut Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn with_num_retention_days(&mut self, value: i64) -> &mut Self {
        self.num_retention_days = Some(value);
        self
    }
}
