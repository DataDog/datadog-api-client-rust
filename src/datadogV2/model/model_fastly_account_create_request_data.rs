// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountCreateRequestData {
    /// Attributes object for creating a Fastly account.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::FastlyAccountCreateRequestAttributes>,
    /// The JSON:API type for this API. Should always be `fastly-accounts`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::FastlyAccountType,
}

impl FastlyAccountCreateRequestData {
    /// Data object for creating a Fastly account.
    pub fn new(
        attributes: crate::datadogV2::model::FastlyAccountCreateRequestAttributes,
        type_: crate::datadogV2::model::FastlyAccountType,
    ) -> FastlyAccountCreateRequestData {
        FastlyAccountCreateRequestData {
            attributes: Box::new(attributes),
            type_,
        }
    }
}
