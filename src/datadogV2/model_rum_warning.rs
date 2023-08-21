// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMWarning {
    /// A unique code for this type of warning.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: String,
    /// A detailed explanation of this specific warning.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: String,
    /// A short human-readable summary of the warning.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}

