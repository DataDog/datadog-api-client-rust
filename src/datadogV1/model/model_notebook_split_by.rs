// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing how to split the graph to display multiple visualizations per request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookSplitBy {
    /// Keys to split on.
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
    /// Tags to split on.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl NotebookSplitBy {
    pub fn new(keys: Vec<String>, tags: Vec<String>) -> NotebookSplitBy {
        NotebookSplitBy { keys, tags }
    }
}
