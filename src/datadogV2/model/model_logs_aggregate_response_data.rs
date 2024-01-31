// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The query results
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateResponseData {
    /// The list of matching buckets, one item per bucket
    #[serde(rename = "buckets")]
    pub buckets: Option<Vec<crate::datadogV2::model::LogsAggregateBucket>>,
}

impl LogsAggregateResponseData {
    pub fn new() -> LogsAggregateResponseData {
        LogsAggregateResponseData { buckets: None }
    }

    pub fn with_buckets(
        &mut self,
        value: Vec<crate::datadogV2::model::LogsAggregateBucket>,
    ) -> &mut Self {
        self.buckets = Some(value);
        self
    }
}
impl Default for LogsAggregateResponseData {
    fn default() -> Self {
        Self::new()
    }
}
