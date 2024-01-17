// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the SSL certificate used for the test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSSLCertificateSubject {
    /// Country Name associated with the certificate.
    #[serde(rename = "C")]
    pub c: Option<String>,
    /// Common Name that associated with the certificate.
    #[serde(rename = "CN")]
    pub cn: Option<String>,
    /// Locality associated with the certificate.
    #[serde(rename = "L")]
    pub l: Option<String>,
    /// Organization associated with the certificate.
    #[serde(rename = "O")]
    pub o: Option<String>,
    /// Organizational Unit associated with the certificate.
    #[serde(rename = "OU")]
    pub ou: Option<String>,
    /// State Or Province Name associated with the certificate.
    #[serde(rename = "ST")]
    pub st: Option<String>,
    /// Subject Alternative Name associated with the certificate.
    #[serde(rename = "altName")]
    pub alt_name: Option<String>,
}

impl SyntheticsSSLCertificateSubject {
    pub fn new() -> SyntheticsSSLCertificateSubject {
        SyntheticsSSLCertificateSubject {
            c: None,
            cn: None,
            l: None,
            o: None,
            ou: None,
            st: None,
            alt_name: None,
        }
    }
}
