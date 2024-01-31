// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Info on your service account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccount {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::GCPSTSServiceAccountAttributes>,
    /// Your service account's unique ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Additional information related to your service account.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::GCPServiceAccountMeta>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
}

impl GCPSTSServiceAccount {
    pub fn new() -> GCPSTSServiceAccount {
        GCPSTSServiceAccount {
            attributes: None,
            id: None,
            meta: None,
            type_: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::GCPSTSServiceAccountAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::GCPServiceAccountMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }

    pub fn with_type_(
        &mut self,
        value: crate::datadogV2::model::GCPServiceAccountType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for GCPSTSServiceAccount {
    fn default() -> Self {
        Self::new()
    }
}
