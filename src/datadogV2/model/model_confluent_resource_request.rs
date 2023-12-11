// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API request for updating a Confluent resource.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfluentResourceRequest {
    /// JSON:API request for updating a Confluent resource.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::ConfluentResourceRequestData>,
}

impl ConfluentResourceRequest {
    pub fn new(
        data: crate::datadogV2::model::ConfluentResourceRequestData,
    ) -> ConfluentResourceRequest {
        ConfluentResourceRequest {
            data: Box::new(data),
        }
    }
}
