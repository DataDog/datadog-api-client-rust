// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A tag filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSTagFilter {
    /// The namespace associated with the tag filter entry.
    #[serde(rename = "namespace")]
    pub namespace: Option<crate::datadogV1::model::AWSNamespace>,
    /// The tag filter string.
    #[serde(rename = "tag_filter_str")]
    pub tag_filter_str: Option<String>,
}

impl AWSTagFilter {
    pub fn new() -> AWSTagFilter {
        AWSTagFilter {
            namespace: None,
            tag_filter_str: None,
        }
    }

    pub fn namespace(mut self, value: crate::datadogV1::model::AWSNamespace) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn tag_filter_str(mut self, value: String) -> Self {
        self.tag_filter_str = Some(value);
        self
    }
}

impl Default for AWSTagFilter {
    fn default() -> Self {
        Self::new()
    }
}
