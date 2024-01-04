// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the issuer of a SSL certificate.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSSLCertificateIssuer {
    /// Country Name that issued the certificate.
    #[serde(rename = "C")]
    pub c: Option<String>,
    /// Common Name that issued certificate.
    #[serde(rename = "CN")]
    pub cn: Option<String>,
    /// Locality that issued the certificate.
    #[serde(rename = "L")]
    pub l: Option<String>,
    /// Organization that issued the certificate.
    #[serde(rename = "O")]
    pub o: Option<String>,
    /// Organizational Unit that issued the certificate.
    #[serde(rename = "OU")]
    pub ou: Option<String>,
    /// State Or Province Name that issued the certificate.
    #[serde(rename = "ST")]
    pub st: Option<String>,
}

impl SyntheticsSSLCertificateIssuer {
    pub fn new() -> SyntheticsSSLCertificateIssuer {
        SyntheticsSSLCertificateIssuer {
            c: None,
            cn: None,
            l: None,
            o: None,
            ou: None,
            st: None,
        }
    }
}
