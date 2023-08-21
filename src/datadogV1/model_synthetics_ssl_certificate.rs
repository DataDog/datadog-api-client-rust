// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSSLCertificate {
    /// Cipher used for the connection.
    #[serde(rename = "cipher", skip_serializing_if = "Option::is_none")]
    pub cipher: String,
    /// Exponent associated to the certificate.
    #[serde(rename = "exponent", skip_serializing_if = "Option::is_none")]
    pub exponent: f64,
    /// Array of extensions and details used for the certificate.
    #[serde(rename = "extKeyUsage", skip_serializing_if = "Option::is_none")]
    pub ext_key_usage: Vec<String>,
    /// MD5 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: String,
    /// SHA-1 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint256", skip_serializing_if = "Option::is_none")]
    pub fingerprint256: String,
    /// Object describing the issuer of a SSL certificate.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: SyntheticsSSLCertificateIssuer,
    /// Modulus associated to the SSL certificate private key.
    #[serde(rename = "modulus", skip_serializing_if = "Option::is_none")]
    pub modulus: String,
    /// TLS protocol used for the test.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: String,
    /// Serial Number assigned by Symantec to the SSL certificate.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: String,
    /// Object describing the SSL certificate used for the test.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: SyntheticsSSLCertificateSubject,
    /// Date from which the SSL certificate is valid.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: String,
    /// Date until which the SSL certificate is valid.
    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: String,
}

