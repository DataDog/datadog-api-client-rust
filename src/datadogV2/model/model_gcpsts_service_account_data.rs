// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountData {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::GCPSTSServiceAccountAttributes>>,
    /// The type of account.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
