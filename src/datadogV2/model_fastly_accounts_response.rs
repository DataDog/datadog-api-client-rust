// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountsResponse {
    /// The JSON:API data schema.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::datadogV2::FastlyAccountResponseData>>,
}

impl FastlyAccountsResponse {
    /// The expected response schema when getting Fastly accounts.
    pub fn new() -> FastlyAccountsResponse {
        FastlyAccountsResponse { data: None }
    }
}
