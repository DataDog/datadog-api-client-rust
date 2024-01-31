// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Client certificate to use when performing the test request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequestCertificate {
    /// Define a request certificate.
    #[serde(rename = "cert")]
    pub cert: Option<crate::datadogV1::model::SyntheticsTestRequestCertificateItem>,
    /// Define a request certificate.
    #[serde(rename = "key")]
    pub key: Option<crate::datadogV1::model::SyntheticsTestRequestCertificateItem>,
}

impl SyntheticsTestRequestCertificate {
    pub fn new() -> SyntheticsTestRequestCertificate {
        SyntheticsTestRequestCertificate {
            cert: None,
            key: None,
        }
    }

    pub fn with_cert(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestCertificateItem,
    ) -> &mut Self {
        self.cert = Some(value);
        self
    }

    pub fn with_key(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestCertificateItem,
    ) -> &mut Self {
        self.key = Some(value);
        self
    }
}
impl Default for SyntheticsTestRequestCertificate {
    fn default() -> Self {
        Self::new()
    }
}
