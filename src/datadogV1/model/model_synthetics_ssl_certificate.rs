// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the SSL certificate used for a Synthetic test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSSLCertificate {
    /// Cipher used for the connection.
    #[serde(rename = "cipher")]
    pub cipher: Option<String>,
    /// Exponent associated to the certificate.
    #[serde(rename = "exponent")]
    pub exponent: Option<f64>,
    /// Array of extensions and details used for the certificate.
    #[serde(rename = "extKeyUsage")]
    pub ext_key_usage: Option<Vec<String>>,
    /// MD5 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    /// SHA-1 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint256")]
    pub fingerprint256: Option<String>,
    /// Object describing the issuer of a SSL certificate.
    #[serde(rename = "issuer")]
    pub issuer: Option<Box<crate::datadogV1::model::SyntheticsSSLCertificateIssuer>>,
    /// Modulus associated to the SSL certificate private key.
    #[serde(rename = "modulus")]
    pub modulus: Option<String>,
    /// TLS protocol used for the test.
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// Serial Number assigned by Symantec to the SSL certificate.
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,
    /// Object describing the SSL certificate used for the test.
    #[serde(rename = "subject")]
    pub subject: Option<Box<crate::datadogV1::model::SyntheticsSSLCertificateSubject>>,
    /// Date from which the SSL certificate is valid.
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    /// Date until which the SSL certificate is valid.
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
}

impl SyntheticsSSLCertificate {
    pub fn new() -> SyntheticsSSLCertificate {
        SyntheticsSSLCertificate {
            cipher: None,
            exponent: None,
            ext_key_usage: None,
            fingerprint: None,
            fingerprint256: None,
            issuer: None,
            modulus: None,
            protocol: None,
            serial_number: None,
            subject: None,
            valid_from: None,
            valid_to: None,
        }
    }
}
