// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes object for updating a Fastly account.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountUpdateRequestAttributes {
    /// The API key of the Fastly account.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
}

impl FastlyAccountUpdateRequestAttributes {
    pub fn new() -> FastlyAccountUpdateRequestAttributes {
        FastlyAccountUpdateRequestAttributes { api_key: None }
    }
}
