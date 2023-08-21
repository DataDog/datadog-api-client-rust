// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialApplicationKeyResponse {
    /// Partial Datadog application key.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: PartialApplicationKey,
    /// Array of objects related to the application key.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<ApplicationKeyResponseIncludedItem>,
}

