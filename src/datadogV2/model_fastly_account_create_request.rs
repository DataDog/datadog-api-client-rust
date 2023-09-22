// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountCreateRequest {
    /// Data object for creating a Fastly account.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::FastlyAccountCreateRequestData>,
}

impl FastlyAccountCreateRequest {
    /// Payload schema when adding a Fastly account.
    pub fn new(
        data: crate::datadogV2::FastlyAccountCreateRequestData,
    ) -> FastlyAccountCreateRequest {
        FastlyAccountCreateRequest {
            data: Box::new(data),
        }
    }
}
