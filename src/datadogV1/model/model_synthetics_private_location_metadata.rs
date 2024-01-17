// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing metadata about the private location.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationMetadata {
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
}

impl SyntheticsPrivateLocationMetadata {
    pub fn new() -> SyntheticsPrivateLocationMetadata {
        SyntheticsPrivateLocationMetadata {
            restricted_roles: None,
        }
    }
}
impl Default for SyntheticsPrivateLocationMetadata {
    fn default() -> Self {
        Self::new()
    }
}
