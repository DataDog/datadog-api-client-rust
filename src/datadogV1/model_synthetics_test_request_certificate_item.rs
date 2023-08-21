// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequestCertificateItem {
    /// Content of the certificate or key.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: String,
    /// File name for the certificate or key.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: String,
    /// Date of update of the certificate or key, ISO format.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: String,
}

