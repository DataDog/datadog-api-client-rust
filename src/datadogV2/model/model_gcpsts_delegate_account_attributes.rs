// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Your delegate account attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCPSTSDelegateAccountAttributes {
    /// Your organization's Datadog principal email address.
    #[serde(rename = "delegate_account_email")]
    pub delegate_account_email: Option<String>,
}

impl GCPSTSDelegateAccountAttributes {
    pub fn new() -> GCPSTSDelegateAccountAttributes {
        GCPSTSDelegateAccountAttributes {
            delegate_account_email: None,
        }
    }
}
impl Default for GCPSTSDelegateAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}
