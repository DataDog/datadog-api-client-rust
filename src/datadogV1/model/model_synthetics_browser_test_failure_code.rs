// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsBrowserTestFailureCode {
    API_REQUEST_FAILURE,
    ASSERTION_FAILURE,
    DOWNLOAD_FILE_TOO_LARGE,
    ELEMENT_NOT_INTERACTABLE,
    EMAIL_VARIABLE_NOT_DEFINED,
    EVALUATE_JAVASCRIPT,
    EVALUATE_JAVASCRIPT_CONTEXT,
    EXTRACT_VARIABLE,
    FORBIDDEN_URL,
    FRAME_DETACHED,
    INCONSISTENCIES,
    INTERNAL_ERROR,
    INVALID_TYPE_TEXT_DELAY,
    INVALID_URL,
    INVALID_VARIABLE_PATTERN,
    INVISIBLE_ELEMENT,
    LOCATE_ELEMENT,
    NAVIGATE_TO_LINK,
    OPEN_URL,
    PRESS_KEY,
    SERVER_CERTIFICATE,
    SELECT_OPTION,
    STEP_TIMEOUT,
    SUB_TEST_NOT_PASSED,
    TEST_TIMEOUT,
    TOO_MANY_HTTP_REQUESTS,
    UNAVAILABLE_BROWSER,
    UNKNOWN,
    UNSUPPORTED_AUTH_SCHEMA,
    UPLOAD_FILES_ELEMENT_TYPE,
    UPLOAD_FILES_DIALOG,
    UPLOAD_FILES_DYNAMIC_ELEMENT,
    UPLOAD_FILES_NAME,
}

impl ToString for SyntheticsBrowserTestFailureCode {
    fn to_string(&self) -> String {
        match self {
            Self::API_REQUEST_FAILURE => String::from("API_REQUEST_FAILURE"),
            Self::ASSERTION_FAILURE => String::from("ASSERTION_FAILURE"),
            Self::DOWNLOAD_FILE_TOO_LARGE => String::from("DOWNLOAD_FILE_TOO_LARGE"),
            Self::ELEMENT_NOT_INTERACTABLE => String::from("ELEMENT_NOT_INTERACTABLE"),
            Self::EMAIL_VARIABLE_NOT_DEFINED => String::from("EMAIL_VARIABLE_NOT_DEFINED"),
            Self::EVALUATE_JAVASCRIPT => String::from("EVALUATE_JAVASCRIPT"),
            Self::EVALUATE_JAVASCRIPT_CONTEXT => String::from("EVALUATE_JAVASCRIPT_CONTEXT"),
            Self::EXTRACT_VARIABLE => String::from("EXTRACT_VARIABLE"),
            Self::FORBIDDEN_URL => String::from("FORBIDDEN_URL"),
            Self::FRAME_DETACHED => String::from("FRAME_DETACHED"),
            Self::INCONSISTENCIES => String::from("INCONSISTENCIES"),
            Self::INTERNAL_ERROR => String::from("INTERNAL_ERROR"),
            Self::INVALID_TYPE_TEXT_DELAY => String::from("INVALID_TYPE_TEXT_DELAY"),
            Self::INVALID_URL => String::from("INVALID_URL"),
            Self::INVALID_VARIABLE_PATTERN => String::from("INVALID_VARIABLE_PATTERN"),
            Self::INVISIBLE_ELEMENT => String::from("INVISIBLE_ELEMENT"),
            Self::LOCATE_ELEMENT => String::from("LOCATE_ELEMENT"),
            Self::NAVIGATE_TO_LINK => String::from("NAVIGATE_TO_LINK"),
            Self::OPEN_URL => String::from("OPEN_URL"),
            Self::PRESS_KEY => String::from("PRESS_KEY"),
            Self::SERVER_CERTIFICATE => String::from("SERVER_CERTIFICATE"),
            Self::SELECT_OPTION => String::from("SELECT_OPTION"),
            Self::STEP_TIMEOUT => String::from("STEP_TIMEOUT"),
            Self::SUB_TEST_NOT_PASSED => String::from("SUB_TEST_NOT_PASSED"),
            Self::TEST_TIMEOUT => String::from("TEST_TIMEOUT"),
            Self::TOO_MANY_HTTP_REQUESTS => String::from("TOO_MANY_HTTP_REQUESTS"),
            Self::UNAVAILABLE_BROWSER => String::from("UNAVAILABLE_BROWSER"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UNSUPPORTED_AUTH_SCHEMA => String::from("UNSUPPORTED_AUTH_SCHEMA"),
            Self::UPLOAD_FILES_ELEMENT_TYPE => String::from("UPLOAD_FILES_ELEMENT_TYPE"),
            Self::UPLOAD_FILES_DIALOG => String::from("UPLOAD_FILES_DIALOG"),
            Self::UPLOAD_FILES_DYNAMIC_ELEMENT => String::from("UPLOAD_FILES_DYNAMIC_ELEMENT"),
            Self::UPLOAD_FILES_NAME => String::from("UPLOAD_FILES_NAME"),
        }
    }
}

impl Serialize for SyntheticsBrowserTestFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTestFailureCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "API_REQUEST_FAILURE" => Self::API_REQUEST_FAILURE,
            "ASSERTION_FAILURE" => Self::ASSERTION_FAILURE,
            "DOWNLOAD_FILE_TOO_LARGE" => Self::DOWNLOAD_FILE_TOO_LARGE,
            "ELEMENT_NOT_INTERACTABLE" => Self::ELEMENT_NOT_INTERACTABLE,
            "EMAIL_VARIABLE_NOT_DEFINED" => Self::EMAIL_VARIABLE_NOT_DEFINED,
            "EVALUATE_JAVASCRIPT" => Self::EVALUATE_JAVASCRIPT,
            "EVALUATE_JAVASCRIPT_CONTEXT" => Self::EVALUATE_JAVASCRIPT_CONTEXT,
            "EXTRACT_VARIABLE" => Self::EXTRACT_VARIABLE,
            "FORBIDDEN_URL" => Self::FORBIDDEN_URL,
            "FRAME_DETACHED" => Self::FRAME_DETACHED,
            "INCONSISTENCIES" => Self::INCONSISTENCIES,
            "INTERNAL_ERROR" => Self::INTERNAL_ERROR,
            "INVALID_TYPE_TEXT_DELAY" => Self::INVALID_TYPE_TEXT_DELAY,
            "INVALID_URL" => Self::INVALID_URL,
            "INVALID_VARIABLE_PATTERN" => Self::INVALID_VARIABLE_PATTERN,
            "INVISIBLE_ELEMENT" => Self::INVISIBLE_ELEMENT,
            "LOCATE_ELEMENT" => Self::LOCATE_ELEMENT,
            "NAVIGATE_TO_LINK" => Self::NAVIGATE_TO_LINK,
            "OPEN_URL" => Self::OPEN_URL,
            "PRESS_KEY" => Self::PRESS_KEY,
            "SERVER_CERTIFICATE" => Self::SERVER_CERTIFICATE,
            "SELECT_OPTION" => Self::SELECT_OPTION,
            "STEP_TIMEOUT" => Self::STEP_TIMEOUT,
            "SUB_TEST_NOT_PASSED" => Self::SUB_TEST_NOT_PASSED,
            "TEST_TIMEOUT" => Self::TEST_TIMEOUT,
            "TOO_MANY_HTTP_REQUESTS" => Self::TOO_MANY_HTTP_REQUESTS,
            "UNAVAILABLE_BROWSER" => Self::UNAVAILABLE_BROWSER,
            "UNKNOWN" => Self::UNKNOWN,
            "UNSUPPORTED_AUTH_SCHEMA" => Self::UNSUPPORTED_AUTH_SCHEMA,
            "UPLOAD_FILES_ELEMENT_TYPE" => Self::UPLOAD_FILES_ELEMENT_TYPE,
            "UPLOAD_FILES_DIALOG" => Self::UPLOAD_FILES_DIALOG,
            "UPLOAD_FILES_DYNAMIC_ELEMENT" => Self::UPLOAD_FILES_DYNAMIC_ELEMENT,
            "UPLOAD_FILES_NAME" => Self::UPLOAD_FILES_NAME,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
