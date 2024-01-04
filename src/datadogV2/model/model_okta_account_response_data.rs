// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object of an Okta account
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccountResponseData {
    /// Attributes object for an Okta account.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::OktaAccountAttributes>,
    /// The ID of the Okta account, a UUID hash of the account name.
    #[serde(rename = "id")]
    pub id: String,
    /// Account type for an Okta account.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OktaAccountType,
}

impl OktaAccountResponseData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::OktaAccountAttributes>,
        id: String,
        type_: crate::datadogV2::model::OktaAccountType,
    ) -> OktaAccountResponseData {
        OktaAccountResponseData {
            attributes,
            id,
            type_,
        }
    }
}
