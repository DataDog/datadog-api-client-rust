// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountUpdateRequestData {
    /// Attributes object for updating a Fastly account.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::FastlyAccountUpdateRequestAttributes>>,
    /// The JSON:API type for this API. Should always be `fastly-accounts`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<crate::datadogV2::model::FastlyAccountType>,
}

impl FastlyAccountUpdateRequestData {
    /// Data object for updating a Fastly account.
    pub fn new() -> FastlyAccountUpdateRequestData {
        FastlyAccountUpdateRequestData {
            attributes: None,
            type_: Some(crate::datadogV2::model::FastlyAccountType::FASTLY_ACCOUNTS),
        }
    }
}
