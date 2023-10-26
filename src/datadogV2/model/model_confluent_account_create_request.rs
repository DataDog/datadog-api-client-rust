// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentAccountCreateRequest {
    /// The data body for adding a Confluent account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::ConfluentAccountCreateRequestData>,
}

impl ConfluentAccountCreateRequest {
    /// Payload schema when adding a Confluent account.
    pub fn new(data: crate::datadogV2::model::ConfluentAccountCreateRequestData) -> ConfluentAccountCreateRequest {
        ConfluentAccountCreateRequest { data: Box::new(data) }
    }
}
