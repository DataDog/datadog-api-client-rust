// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object for updating a Fastly account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FastlyAccountUpdateRequestData {
    /// Attributes object for updating a Fastly account.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::FastlyAccountUpdateRequestAttributes>>,
    /// The JSON:API type for this API. Should always be `fastly-accounts`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::FastlyAccountType>,
}

impl FastlyAccountUpdateRequestData {
    pub fn new() -> FastlyAccountUpdateRequestData {
        FastlyAccountUpdateRequestData {
            attributes: None,
            type_: None,
        }
    }
}
