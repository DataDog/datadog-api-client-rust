// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define a request certificate.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequestCertificateItem {
    /// Content of the certificate or key.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// File name for the certificate or key.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// Date of update of the certificate or key, ISO format.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

impl SyntheticsTestRequestCertificateItem {
    pub fn new() -> SyntheticsTestRequestCertificateItem {
        SyntheticsTestRequestCertificateItem {
            content: None,
            filename: None,
            updated_at: None,
        }
    }
}
