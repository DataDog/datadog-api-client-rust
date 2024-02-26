// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The query being made on the logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinitionSearch {
    /// Search value to apply.
    #[serde(rename = "query")]
    pub query: String,
}

impl LogQueryDefinitionSearch {
    pub fn new(query: String) -> LogQueryDefinitionSearch {
        LogQueryDefinitionSearch { query }
    }
}
