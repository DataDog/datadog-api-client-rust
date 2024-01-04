// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Errors occurred.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPCIAppErrors {
    /// Structured errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV2::model::HTTPCIAppError>>,
}

impl HTTPCIAppErrors {
    pub fn new() -> HTTPCIAppErrors {
        HTTPCIAppErrors { errors: None }
    }
}