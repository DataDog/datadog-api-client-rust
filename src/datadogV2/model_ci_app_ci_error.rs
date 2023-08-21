// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCIError {
    /// Error category used to differentiate between issues related to the developer or provider environments.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: CIAppCIErrorDomain,
    /// Error message.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// The stack trace of the reported errors.
    #[serde(rename = "stack", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub stack: Option<String>,
    /// Short description of the error type.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub type_: Option<String>,
}

