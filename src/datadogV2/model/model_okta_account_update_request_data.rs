// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object for updating an Okta account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OktaAccountUpdateRequestData {
    /// Attributes object for updating an Okta account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::OktaAccountUpdateRequestAttributes>,
    /// Account type for an Okta account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OktaAccountType>,
}

impl OktaAccountUpdateRequestData {
    pub fn new() -> OktaAccountUpdateRequestData {
        OktaAccountUpdateRequestData {
            attributes: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::OktaAccountUpdateRequestAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::OktaAccountType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for OktaAccountUpdateRequestData {
    fn default() -> Self {
        Self::new()
    }
}
