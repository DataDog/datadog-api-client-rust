// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the definition of a metric's tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAllTagsAttributes {
    /// List of indexed tag value pairs.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl MetricAllTagsAttributes {
    pub fn new() -> MetricAllTagsAttributes {
        MetricAllTagsAttributes { tags: None }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for MetricAllTagsAttributes {
    fn default() -> Self {
        Self::new()
    }
}
