// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Invalid query performed.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogErrors {
    /// Structured errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV2::model::HTTPLogError>>,
}

impl HTTPLogErrors {
    pub fn new() -> HTTPLogErrors {
        HTTPLogErrors { errors: None }
    }

    pub fn with_errors(&mut self, value: Vec<crate::datadogV2::model::HTTPLogError>) -> &mut Self {
        self.errors = Some(value);
        self
    }
}
impl Default for HTTPLogErrors {
    fn default() -> Self {
        Self::new()
    }
}
