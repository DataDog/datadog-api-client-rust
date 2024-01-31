// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options for the Global Variable for MFA.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableOptions {
    /// Parameters for the TOTP/MFA variable
    #[serde(rename = "totp_parameters")]
    pub totp_parameters: Option<crate::datadogV1::model::SyntheticsGlobalVariableTOTPParameters>,
}

impl SyntheticsGlobalVariableOptions {
    pub fn new() -> SyntheticsGlobalVariableOptions {
        SyntheticsGlobalVariableOptions {
            totp_parameters: None,
        }
    }

    pub fn with_totp_parameters(
        &mut self,
        value: crate::datadogV1::model::SyntheticsGlobalVariableTOTPParameters,
    ) -> &mut Self {
        self.totp_parameters = Some(value);
        self
    }
}
impl Default for SyntheticsGlobalVariableOptions {
    fn default() -> Self {
        Self::new()
    }
}
