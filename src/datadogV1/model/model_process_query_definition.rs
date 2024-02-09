// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The process query to use in the widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessQueryDefinition {
    /// List of processes.
    #[serde(rename = "filter_by")]
    pub filter_by: Option<Vec<String>>,
    /// Max number of items in the filter list.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Your chosen metric.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Your chosen search term.
    #[serde(rename = "search_by")]
    pub search_by: Option<String>,
}

impl ProcessQueryDefinition {
    pub fn new(metric: String) -> ProcessQueryDefinition {
        ProcessQueryDefinition {
            filter_by: None,
            limit: None,
            metric,
            search_by: None,
        }
    }

    pub fn filter_by(&mut self, value: Vec<String>) -> &mut Self {
        self.filter_by = Some(value);
        self
    }

    pub fn limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn search_by(&mut self, value: String) -> &mut Self {
        self.search_by = Some(value);
        self
    }
}
