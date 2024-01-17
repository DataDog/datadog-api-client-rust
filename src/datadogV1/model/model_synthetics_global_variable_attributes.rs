// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the global variable.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableAttributes {
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles")]
    pub restricted_roles: Option<Vec<String>>,
}

impl SyntheticsGlobalVariableAttributes {
    pub fn new() -> SyntheticsGlobalVariableAttributes {
        SyntheticsGlobalVariableAttributes {
            restricted_roles: None,
        }
    }
}
impl Default for SyntheticsGlobalVariableAttributes {
    fn default() -> Self {
        Self::new()
    }
}
