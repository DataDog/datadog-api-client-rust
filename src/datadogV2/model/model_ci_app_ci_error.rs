// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Contains information of the CI error.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCIError {
    /// Error category used to differentiate between issues related to the developer or provider environments.
    #[serde(rename = "domain")]
    pub domain: Option<crate::datadogV2::model::CIAppCIErrorDomain>,
    /// Error message.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub message: Option<Option<String>>,
    /// The stack trace of the reported errors.
    #[serde(rename = "stack", default, with = "::serde_with::rust::double_option")]
    pub stack: Option<Option<String>>,
    /// Short description of the error type.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option")]
    pub type_: Option<Option<String>>,
}

impl CIAppCIError {
    pub fn new() -> CIAppCIError {
        CIAppCIError {
            domain: None,
            message: None,
            stack: None,
            type_: None,
        }
    }

    pub fn domain(&mut self, value: crate::datadogV2::model::CIAppCIErrorDomain) -> &mut Self {
        self.domain = Some(value);
        self
    }

    pub fn message(&mut self, value: Option<String>) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn stack(&mut self, value: Option<String>) -> &mut Self {
        self.stack = Some(value);
        self
    }

    pub fn type_(&mut self, value: Option<String>) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for CIAppCIError {
    fn default() -> Self {
        Self::new()
    }
}
