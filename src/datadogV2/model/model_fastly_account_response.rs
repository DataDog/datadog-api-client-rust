// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountResponse {
    /// Data object of a Fastly account.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::datadogV2::model::FastlyAccountResponseData>>,
}

impl FastlyAccountResponse {
    /// The expected response schema when getting a Fastly account.
    pub fn new() -> FastlyAccountResponse {
        FastlyAccountResponse { data: None }
    }
}