// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Payload schema when updating an Okta account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccountUpdateRequest {
    /// Data object for updating an Okta account.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::OktaAccountUpdateRequestData,
}

impl OktaAccountUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::OktaAccountUpdateRequestData,
    ) -> OktaAccountUpdateRequest {
        OktaAccountUpdateRequest { data }
    }
}
