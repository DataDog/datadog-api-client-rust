// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountData {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::GCPSTSServiceAccountAttributes>>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
}

impl GCPSTSServiceAccountData {
    /// Additional metadata on your generated service account.
    pub fn new() -> GCPSTSServiceAccountData {
        GCPSTSServiceAccountData {
            attributes: None,
            type_: None,
        }
    }
}
