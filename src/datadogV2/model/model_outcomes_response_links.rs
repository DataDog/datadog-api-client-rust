// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Links attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesResponseLinks {
    /// Link for the next set of results.
    #[serde(rename = "next")]
    pub next: Option<String>,
}

impl OutcomesResponseLinks {
    pub fn new() -> OutcomesResponseLinks {
        OutcomesResponseLinks { next: None }
    }

    pub fn with_next(&mut self, value: String) -> &mut Self {
        self.next = Some(value);
        self
    }
}
impl Default for OutcomesResponseLinks {
    fn default() -> Self {
        Self::new()
    }
}
