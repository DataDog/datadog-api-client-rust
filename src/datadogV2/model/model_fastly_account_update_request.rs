// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountUpdateRequest {
    /// Data object for updating a Fastly account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::FastlyAccountUpdateRequestData>,
}

impl FastlyAccountUpdateRequest {
    /// Payload schema when updating a Fastly account.
    pub fn new(
        data: crate::datadogV2::model::FastlyAccountUpdateRequestData,
    ) -> FastlyAccountUpdateRequest {
        FastlyAccountUpdateRequest {
            data: Box::new(data),
        }
    }
}
