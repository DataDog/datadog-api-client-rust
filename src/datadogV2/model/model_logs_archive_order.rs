// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A ordered list of archive IDs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsArchiveOrder {
    /// The definition of an archive order.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::LogsArchiveOrderDefinition>,
}

impl LogsArchiveOrder {
    pub fn new() -> LogsArchiveOrder {
        LogsArchiveOrder { data: None }
    }

    pub fn data(mut self, value: crate::datadogV2::model::LogsArchiveOrderDefinition) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for LogsArchiveOrder {
    fn default() -> Self {
        Self::new()
    }
}
