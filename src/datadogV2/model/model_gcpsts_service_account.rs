// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccount {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::GCPSTSServiceAccountAttributes>>,
    /// Your service account's unique ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Additional information related to your service account.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::GCPServiceAccountMeta>>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
}

impl GCPSTSServiceAccount {
    /// Info on your service account.
    pub fn new() -> GCPSTSServiceAccount {
        GCPSTSServiceAccount {
            attributes: None,
            id: None,
            meta: None,
            type_: None,
        }
    }
}
