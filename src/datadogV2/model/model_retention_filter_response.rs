// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The retention filters definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterResponse {
    /// The definition of the retention filter.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::RetentionFilter>,
}

impl RetentionFilterResponse {
    pub fn new() -> RetentionFilterResponse {
        RetentionFilterResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::RetentionFilter) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for RetentionFilterResponse {
    fn default() -> Self {
        Self::new()
    }
}
