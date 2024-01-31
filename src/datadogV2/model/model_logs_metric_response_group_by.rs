// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A group by rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricResponseGroupBy {
    /// The path to the value the log-based metric will be aggregated over.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Eventual name of the tag that gets created. By default, the path attribute is used as the tag name.
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
}

impl LogsMetricResponseGroupBy {
    pub fn new() -> LogsMetricResponseGroupBy {
        LogsMetricResponseGroupBy {
            path: None,
            tag_name: None,
        }
    }

    pub fn with_path(&mut self, value: String) -> &mut Self {
        self.path = Some(value);
        self
    }

    pub fn with_tag_name(&mut self, value: String) -> &mut Self {
        self.tag_name = Some(value);
        self
    }
}
impl Default for LogsMetricResponseGroupBy {
    fn default() -> Self {
        Self::new()
    }
}
