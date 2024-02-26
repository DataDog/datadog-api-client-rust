// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the ordered list of log index names.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexesOrder {
    /// Array of strings identifying by their name(s) the index(es) of your organization.
    /// Logs are tested against the query filter of each index one by one, following the order of the array.
    /// Logs are eventually stored in the first matching index.
    #[serde(rename = "index_names")]
    pub index_names: Vec<String>,
}

impl LogsIndexesOrder {
    pub fn new(index_names: Vec<String>) -> LogsIndexesOrder {
        LogsIndexesOrder { index_names }
    }
}
