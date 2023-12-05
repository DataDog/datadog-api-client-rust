// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response schema when interacting with a list of Confluent resources.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourcesResponse {
    /// The JSON:API data attribute.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ConfluentResourceResponseData>>,
}

impl ConfluentResourcesResponse {
    pub fn new() -> ConfluentResourcesResponse {
        ConfluentResourcesResponse { data: None }
    }
}