// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Datadog principal service account info.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccount {
    /// Your delegate account attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::GCPSTSDelegateAccountAttributes>,
    /// The ID of the delegate service account.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPSTSDelegateAccountType>,
}

impl GCPSTSDelegateAccount {
    pub fn new() -> GCPSTSDelegateAccount {
        GCPSTSDelegateAccount {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::GCPSTSDelegateAccountAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::GCPSTSDelegateAccountType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for GCPSTSDelegateAccount {
    fn default() -> Self {
        Self::new()
    }
}
