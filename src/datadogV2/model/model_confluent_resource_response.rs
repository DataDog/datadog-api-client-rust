// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response schema when interacting with a Confluent resource.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceResponse {
    /// Confluent Cloud resource data.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::ConfluentResourceResponseData>>,
}

impl ConfluentResourceResponse {
    pub fn new() -> ConfluentResourceResponse {
        ConfluentResourceResponse { data: None }
    }
}