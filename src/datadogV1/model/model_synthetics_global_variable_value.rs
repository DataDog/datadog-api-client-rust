// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Value of the global variable.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableValue {
    /// Options for the Global Variable for MFA.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV1::model::SyntheticsGlobalVariableOptions>,
    /// Determines if the value of the variable is hidden.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Value of the global variable. When reading a global variable,
    /// the value will not be present if the variable is hidden with the `secure` property.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

impl SyntheticsGlobalVariableValue {
    pub fn new() -> SyntheticsGlobalVariableValue {
        SyntheticsGlobalVariableValue {
            options: None,
            secure: None,
            value: None,
        }
    }

    pub fn with_options(
        &mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableOptions,
    ) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn with_secure(&mut self, value: bool) -> &mut Self {
        self.secure = Some(value);
        self
    }

    pub fn with_value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
}
impl Default for SyntheticsGlobalVariableValue {
    fn default() -> Self {
        Self::new()
    }
}
