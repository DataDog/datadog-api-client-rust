// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective response containing a single service level objective.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOResponse {
    /// A service level objective object includes a service level indicator, thresholds
    /// for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SLOResponseData>,
    /// An array of error messages. Each endpoint documents how/whether this field is
    /// used.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
}

impl SLOResponse {
    pub fn new() -> SLOResponse {
        SLOResponse {
            data: None,
            errors: None,
        }
    }

    pub fn data(mut self, value: crate::datadogV1::model::SLOResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }
}

impl Default for SLOResponse {
    fn default() -> Self {
        Self::new()
    }
}
