// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsMetricGroupBy {
    /// The path to the value the log-based metric will be aggregated over.
    #[serde(rename = "path")]
    pub path: String,
    /// Eventual name of the tag that gets created. By default, the path attribute is used as the tag name.
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
}

impl LogsMetricGroupBy {
    /// A group by rule.
    pub fn new(path: String) -> LogsMetricGroupBy {
        LogsMetricGroupBy {
            path: path,
            tag_name: None,
        }
    }
}
