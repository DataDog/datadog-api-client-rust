// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to send with the request to retrieve a list of logs from your Organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequest {
    /// The log index on which the request is performed. For multi-index organizations,
    /// the default is all live indexes. Historical indexes of rehydrated logs must be specified.
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// Number of logs return in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The search query - following the log search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Time-ascending `asc` or time-descending `desc` results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::LogsSort>,
    /// Hash identifier of the first log to return in the list, available in a log `id` attribute.
    /// This parameter is used for the pagination feature.
    ///
    /// **Note**: This parameter is ignored if the corresponding log
    /// is out of the scope of the specified time window.
    #[serde(rename = "startAt")]
    pub start_at: Option<String>,
    /// Timeframe to retrieve the log from.
    #[serde(rename = "time")]
    pub time: Box<crate::datadogV1::model::LogsListRequestTime>,
}

impl LogsListRequest {
    pub fn new(time: crate::datadogV1::model::LogsListRequestTime) -> LogsListRequest {
        LogsListRequest {
            index: None,
            limit: None,
            query: None,
            sort: None,
            start_at: None,
            time: Box::new(time),
        }
    }
}
