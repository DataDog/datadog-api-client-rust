// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes object for Fastly service requests.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyServiceAttributes {
    /// A list of tags for the Fastly service.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl FastlyServiceAttributes {
    pub fn new() -> FastlyServiceAttributes {
        FastlyServiceAttributes { tags: None }
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}

impl Default for FastlyServiceAttributes {
    fn default() -> Self {
        Self::new()
    }
}
