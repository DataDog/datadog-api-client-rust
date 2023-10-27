// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricResponseGroupBy {
    /// The path to the value the span-based metric will be aggregated over.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Eventual name of the tag that gets created. By default, the path attribute is used as the tag name.
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
}

impl SpansMetricResponseGroupBy {
    /// A group by rule.
    pub fn new() -> SpansMetricResponseGroupBy {
        SpansMetricResponseGroupBy {
            path: None,
            tag_name: None,
        }
    }
}
