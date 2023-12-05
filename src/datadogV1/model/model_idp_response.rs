// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The IdP response object.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IdpResponse {
    /// Identity provider response.
    #[serde(rename = "message")]
    pub message: String,
}

impl IdpResponse {
    pub fn new(message: String) -> IdpResponse {
        IdpResponse { message }
    }
}