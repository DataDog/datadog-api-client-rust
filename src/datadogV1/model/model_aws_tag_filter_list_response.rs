// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An array of tag filter rules by `namespace` and tag filter string.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSTagFilterListResponse {
    /// An array of tag filters.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV1::model::AWSTagFilter>>,
}

impl AWSTagFilterListResponse {
    pub fn new() -> AWSTagFilterListResponse {
        AWSTagFilterListResponse { filters: None }
    }

    pub fn filters(&mut self, value: Vec<crate::datadogV1::model::AWSTagFilter>) -> &mut Self {
        self.filters = Some(value);
        self
    }
}

impl Default for AWSTagFilterListResponse {
    fn default() -> Self {
        Self::new()
    }
}
