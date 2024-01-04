// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object collecting warnings for a given step.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStepDetailWarning {
    /// Message for the warning.
    #[serde(rename = "message")]
    pub message: String,
    /// User locator used.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsWarningType,
}

impl SyntheticsStepDetailWarning {
    pub fn new(
        message: String,
        type_: crate::datadogV1::model::SyntheticsWarningType,
    ) -> SyntheticsStepDetailWarning {
        SyntheticsStepDetailWarning { message, type_ }
    }
}
