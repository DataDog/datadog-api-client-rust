// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsApiTestFailureCode {
    BODY_TOO_LARGE,
    DENIED,
    TOO_MANY_REDIRECTS,
    AUTHENTICATION_ERROR,
    DECRYPTION,
    INVALID_CHAR_IN_HEADER,
    HEADER_TOO_LARGE,
    HEADERS_INCOMPATIBLE_CONTENT_LENGTH,
    INVALID_REQUEST,
    REQUIRES_UPDATE,
    UNESCAPED_CHARACTERS_IN_REQUEST_PATH,
    MALFORMED_RESPONSE,
    INCORRECT_ASSERTION,
    CONNREFUSED,
    CONNRESET,
    DNS,
    HOSTUNREACH,
    NETUNREACH,
    TIMEOUT,
    SSL,
    OCSP,
    INVALID_TEST,
    TUNNEL,
    WEBSOCKET,
    UNKNOWN,
    INTERNAL_ERROR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsApiTestFailureCode {
    fn to_string(&self) -> String {
        match self {
            Self::BODY_TOO_LARGE => String::from("BODY_TOO_LARGE"),
            Self::DENIED => String::from("DENIED"),
            Self::TOO_MANY_REDIRECTS => String::from("TOO_MANY_REDIRECTS"),
            Self::AUTHENTICATION_ERROR => String::from("AUTHENTICATION_ERROR"),
            Self::DECRYPTION => String::from("DECRYPTION"),
            Self::INVALID_CHAR_IN_HEADER => String::from("INVALID_CHAR_IN_HEADER"),
            Self::HEADER_TOO_LARGE => String::from("HEADER_TOO_LARGE"),
            Self::HEADERS_INCOMPATIBLE_CONTENT_LENGTH => {
                String::from("HEADERS_INCOMPATIBLE_CONTENT_LENGTH")
            }
            Self::INVALID_REQUEST => String::from("INVALID_REQUEST"),
            Self::REQUIRES_UPDATE => String::from("REQUIRES_UPDATE"),
            Self::UNESCAPED_CHARACTERS_IN_REQUEST_PATH => {
                String::from("UNESCAPED_CHARACTERS_IN_REQUEST_PATH")
            }
            Self::MALFORMED_RESPONSE => String::from("MALFORMED_RESPONSE"),
            Self::INCORRECT_ASSERTION => String::from("INCORRECT_ASSERTION"),
            Self::CONNREFUSED => String::from("CONNREFUSED"),
            Self::CONNRESET => String::from("CONNRESET"),
            Self::DNS => String::from("DNS"),
            Self::HOSTUNREACH => String::from("HOSTUNREACH"),
            Self::NETUNREACH => String::from("NETUNREACH"),
            Self::TIMEOUT => String::from("TIMEOUT"),
            Self::SSL => String::from("SSL"),
            Self::OCSP => String::from("OCSP"),
            Self::INVALID_TEST => String::from("INVALID_TEST"),
            Self::TUNNEL => String::from("TUNNEL"),
            Self::WEBSOCKET => String::from("WEBSOCKET"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::INTERNAL_ERROR => String::from("INTERNAL_ERROR"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsApiTestFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsApiTestFailureCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BODY_TOO_LARGE" => Self::BODY_TOO_LARGE,
            "DENIED" => Self::DENIED,
            "TOO_MANY_REDIRECTS" => Self::TOO_MANY_REDIRECTS,
            "AUTHENTICATION_ERROR" => Self::AUTHENTICATION_ERROR,
            "DECRYPTION" => Self::DECRYPTION,
            "INVALID_CHAR_IN_HEADER" => Self::INVALID_CHAR_IN_HEADER,
            "HEADER_TOO_LARGE" => Self::HEADER_TOO_LARGE,
            "HEADERS_INCOMPATIBLE_CONTENT_LENGTH" => Self::HEADERS_INCOMPATIBLE_CONTENT_LENGTH,
            "INVALID_REQUEST" => Self::INVALID_REQUEST,
            "REQUIRES_UPDATE" => Self::REQUIRES_UPDATE,
            "UNESCAPED_CHARACTERS_IN_REQUEST_PATH" => Self::UNESCAPED_CHARACTERS_IN_REQUEST_PATH,
            "MALFORMED_RESPONSE" => Self::MALFORMED_RESPONSE,
            "INCORRECT_ASSERTION" => Self::INCORRECT_ASSERTION,
            "CONNREFUSED" => Self::CONNREFUSED,
            "CONNRESET" => Self::CONNRESET,
            "DNS" => Self::DNS,
            "HOSTUNREACH" => Self::HOSTUNREACH,
            "NETUNREACH" => Self::NETUNREACH,
            "TIMEOUT" => Self::TIMEOUT,
            "SSL" => Self::SSL,
            "OCSP" => Self::OCSP,
            "INVALID_TEST" => Self::INVALID_TEST,
            "TUNNEL" => Self::TUNNEL,
            "WEBSOCKET" => Self::WEBSOCKET,
            "UNKNOWN" => Self::UNKNOWN,
            "INTERNAL_ERROR" => Self::INTERNAL_ERROR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
