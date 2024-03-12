// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The search and filter query settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsQueryFilter {
    /// The minimum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// For customers with multiple indexes, the indexes to search. Defaults to ['*'] which means all indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The search query - following the log search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Specifies storage type as indexes or online-archives
    #[serde(rename = "storage_tier")]
    pub storage_tier: Option<crate::datadogV2::model::LogsStorageTier>,
    /// The maximum time for the requested logs, supports date math and regular timestamps (milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl LogsQueryFilter {
    pub fn new() -> LogsQueryFilter {
        LogsQueryFilter {
            from: None,
            indexes: None,
            query: None,
            storage_tier: None,
            to: None,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn storage_tier(mut self, value: crate::datadogV2::model::LogsStorageTier) -> Self {
        self.storage_tier = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
        self
    }
}

impl Default for LogsQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}
