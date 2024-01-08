// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A JSON array of billing type.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationBilling {
    /// The type of billing. Only `parent_billing` is supported.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl OrganizationBilling {
    pub fn new() -> OrganizationBilling {
        OrganizationBilling { type_: None }
    }
}
impl Default for OrganizationBilling {
    fn default() -> Self {
        Self::new()
    }
}
