// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data on your service account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountUpdateRequestData {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::GCPSTSServiceAccountAttributes>>,
    /// Your service account's unique ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
}

impl GCPSTSServiceAccountUpdateRequestData {
    pub fn new() -> GCPSTSServiceAccountUpdateRequestData {
        GCPSTSServiceAccountUpdateRequestData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for GCPSTSServiceAccountUpdateRequestData {
    fn default() -> Self {
        Self::new()
    }
}
