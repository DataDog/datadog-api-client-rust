// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Absolute timeframe.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookAbsoluteTime {
    /// The end time.
    #[serde(rename = "end")]
    pub end: String,
    /// Indicates whether the timeframe should be shifted to end at the current time.
    #[serde(rename = "live")]
    pub live: Option<bool>,
    /// The start time.
    #[serde(rename = "start")]
    pub start: String,
}

impl NotebookAbsoluteTime {
    pub fn new(end: String, start: String) -> NotebookAbsoluteTime {
        NotebookAbsoluteTime {
            end,
            live: None,
            start,
        }
    }

    pub fn live(&mut self, value: bool) -> &mut Self {
        self.live = Some(value);
        self
    }
}
