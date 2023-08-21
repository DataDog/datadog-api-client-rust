// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSSLCertificateSubject {
    /// Country Name associated with the certificate.
    #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
    pub c: String,
    /// Common Name that associated with the certificate.
    #[serde(rename = "CN", skip_serializing_if = "Option::is_none")]
    pub cn: String,
    /// Locality associated with the certificate.
    #[serde(rename = "L", skip_serializing_if = "Option::is_none")]
    pub l: String,
    /// Organization associated with the certificate.
    #[serde(rename = "O", skip_serializing_if = "Option::is_none")]
    pub o: String,
    /// Organizational Unit associated with the certificate.
    #[serde(rename = "OU", skip_serializing_if = "Option::is_none")]
    pub ou: String,
    /// State Or Province Name associated with the certificate.
    #[serde(rename = "ST", skip_serializing_if = "Option::is_none")]
    pub st: String,
    /// Subject Alternative Name associated with the certificate.
    #[serde(rename = "altName", skip_serializing_if = "Option::is_none")]
    pub alt_name: String,
}

