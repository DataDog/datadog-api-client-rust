// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPLogErrors {
    /// Structured errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::datadogV2::model::HTTPLogError>>,
}

impl HTTPLogErrors {
    /// Invalid query performed.
    pub fn new() -> HTTPLogErrors {
        HTTPLogErrors { errors: None }
    }
}
