// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object of a Fastly account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountResponseData {
    /// Attributes object of a Fastly account.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::FastlyAccounResponseAttributes,
    /// The ID of the Fastly account, a hash of the account name.
    #[serde(rename = "id")]
    pub id: String,
    /// The JSON:API type for this API. Should always be `fastly-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::FastlyAccountType,
}

impl FastlyAccountResponseData {
    pub fn new(
        attributes: crate::datadogV2::model::FastlyAccounResponseAttributes,
        id: String,
        type_: crate::datadogV2::model::FastlyAccountType,
    ) -> FastlyAccountResponseData {
        FastlyAccountResponseData {
            attributes,
            id,
            type_,
        }
    }
}
