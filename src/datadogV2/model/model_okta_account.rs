// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Schema for an Okta account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccount {
    /// Attributes object for an Okta account.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::OktaAccountAttributes,
    /// The ID of the Okta account, a UUID hash of the account name.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Account type for an Okta account.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OktaAccountType,
}

impl OktaAccount {
    pub fn new(
        attributes: crate::datadogV2::model::OktaAccountAttributes,
        type_: crate::datadogV2::model::OktaAccountType,
    ) -> OktaAccount {
        OktaAccount {
            attributes,
            id: None,
            type_,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
}
