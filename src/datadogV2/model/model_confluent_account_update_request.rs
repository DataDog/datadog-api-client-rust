// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountUpdateRequest {
    /// Data object for updating a Confluent account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::ConfluentAccountUpdateRequestData>,
}

impl ConfluentAccountUpdateRequest {
    /// The JSON:API request for updating a Confluent account.
    pub fn new(data: crate::datadogV2::model::ConfluentAccountUpdateRequestData) -> ConfluentAccountUpdateRequest {
        ConfluentAccountUpdateRequest { data: Box::new(data) }
    }
}
