// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsApiTestFailureCode {
    #[serde(rename = "BODY_TOO_LARGE")]
	BODY_TOO_LARGE,
    #[serde(rename = "DENIED")]
	DENIED,
    #[serde(rename = "TOO_MANY_REDIRECTS")]
	TOO_MANY_REDIRECTS,
    #[serde(rename = "AUTHENTICATION_ERROR")]
	AUTHENTICATION_ERROR,
    #[serde(rename = "DECRYPTION")]
	DECRYPTION,
    #[serde(rename = "INVALID_CHAR_IN_HEADER")]
	INVALID_CHAR_IN_HEADER,
    #[serde(rename = "HEADER_TOO_LARGE")]
	HEADER_TOO_LARGE,
    #[serde(rename = "HEADERS_INCOMPATIBLE_CONTENT_LENGTH")]
	HEADERS_INCOMPATIBLE_CONTENT_LENGTH,
    #[serde(rename = "INVALID_REQUEST")]
	INVALID_REQUEST,
    #[serde(rename = "REQUIRES_UPDATE")]
	REQUIRES_UPDATE,
    #[serde(rename = "UNESCAPED_CHARACTERS_IN_REQUEST_PATH")]
	UNESCAPED_CHARACTERS_IN_REQUEST_PATH,
    #[serde(rename = "MALFORMED_RESPONSE")]
	MALFORMED_RESPONSE,
    #[serde(rename = "INCORRECT_ASSERTION")]
	INCORRECT_ASSERTION,
    #[serde(rename = "CONNREFUSED")]
	CONNREFUSED,
    #[serde(rename = "CONNRESET")]
	CONNRESET,
    #[serde(rename = "DNS")]
	DNS,
    #[serde(rename = "HOSTUNREACH")]
	HOSTUNREACH,
    #[serde(rename = "NETUNREACH")]
	NETUNREACH,
    #[serde(rename = "TIMEOUT")]
	TIMEOUT,
    #[serde(rename = "SSL")]
	SSL,
    #[serde(rename = "OCSP")]
	OCSP,
    #[serde(rename = "INVALID_TEST")]
	INVALID_TEST,
    #[serde(rename = "TUNNEL")]
	TUNNEL,
    #[serde(rename = "WEBSOCKET")]
	WEBSOCKET,
    #[serde(rename = "UNKNOWN")]
	UNKNOWN,
    #[serde(rename = "INTERNAL_ERROR")]
	INTERNAL_ERROR,
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
            Self::HEADERS_INCOMPATIBLE_CONTENT_LENGTH => String::from("HEADERS_INCOMPATIBLE_CONTENT_LENGTH"),
            Self::INVALID_REQUEST => String::from("INVALID_REQUEST"),
            Self::REQUIRES_UPDATE => String::from("REQUIRES_UPDATE"),
            Self::UNESCAPED_CHARACTERS_IN_REQUEST_PATH => String::from("UNESCAPED_CHARACTERS_IN_REQUEST_PATH"),
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
        }
    }
}

impl Default for SyntheticsApiTestFailureCode {
    fn default() -> SyntheticsApiTestFailureCode {
        Self::BODY_TOO_LARGE
    }
}
