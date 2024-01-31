// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Additional metadata on your generated service account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSServiceAccountData {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::GCPSTSServiceAccountAttributes>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
}

impl GCPSTSServiceAccountData {
    pub fn new() -> GCPSTSServiceAccountData {
        GCPSTSServiceAccountData {
            attributes: None,
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

    pub fn with_type_(
        &mut self,
        value: crate::datadogV2::model::GCPServiceAccountType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for GCPSTSServiceAccountData {
    fn default() -> Self {
        Self::new()
    }
}
