// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccount {
    /// Your delegate account attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::GCPSTSDelegateAccountAttributes>>,
    /// The ID of the delegate service account.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPSTSDelegateAccountType>,
}

impl GCPSTSDelegateAccount {
    /// Datadog principal service account info.
    pub fn new() -> GCPSTSDelegateAccount {
        GCPSTSDelegateAccount {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
