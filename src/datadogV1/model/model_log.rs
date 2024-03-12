// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a log after being processed and stored by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// JSON object containing all log attributes and their associated values.
    #[serde(rename = "content")]
    pub content: Option<crate::datadogV1::model::LogContent>,
    /// Unique ID of the Log.
    #[serde(rename = "id")]
    pub id: Option<String>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            content: None,
            id: None,
        }
    }

    pub fn content(mut self, value: crate::datadogV1::model::LogContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}
