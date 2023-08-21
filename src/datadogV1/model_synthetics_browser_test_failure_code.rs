// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBrowserTestFailureCode {
    #[serde(rename = "API_REQUEST_FAILURE")]
	API_REQUEST_FAILURE,
    #[serde(rename = "ASSERTION_FAILURE")]
	ASSERTION_FAILURE,
    #[serde(rename = "DOWNLOAD_FILE_TOO_LARGE")]
	DOWNLOAD_FILE_TOO_LARGE,
    #[serde(rename = "ELEMENT_NOT_INTERACTABLE")]
	ELEMENT_NOT_INTERACTABLE,
    #[serde(rename = "EMAIL_VARIABLE_NOT_DEFINED")]
	EMAIL_VARIABLE_NOT_DEFINED,
    #[serde(rename = "EVALUATE_JAVASCRIPT")]
	EVALUATE_JAVASCRIPT,
    #[serde(rename = "EVALUATE_JAVASCRIPT_CONTEXT")]
	EVALUATE_JAVASCRIPT_CONTEXT,
    #[serde(rename = "EXTRACT_VARIABLE")]
	EXTRACT_VARIABLE,
    #[serde(rename = "FORBIDDEN_URL")]
	FORBIDDEN_URL,
    #[serde(rename = "FRAME_DETACHED")]
	FRAME_DETACHED,
    #[serde(rename = "INCONSISTENCIES")]
	INCONSISTENCIES,
    #[serde(rename = "INTERNAL_ERROR")]
	INTERNAL_ERROR,
    #[serde(rename = "INVALID_TYPE_TEXT_DELAY")]
	INVALID_TYPE_TEXT_DELAY,
    #[serde(rename = "INVALID_URL")]
	INVALID_URL,
    #[serde(rename = "INVALID_VARIABLE_PATTERN")]
	INVALID_VARIABLE_PATTERN,
    #[serde(rename = "INVISIBLE_ELEMENT")]
	INVISIBLE_ELEMENT,
    #[serde(rename = "LOCATE_ELEMENT")]
	LOCATE_ELEMENT,
    #[serde(rename = "NAVIGATE_TO_LINK")]
	NAVIGATE_TO_LINK,
    #[serde(rename = "OPEN_URL")]
	OPEN_URL,
    #[serde(rename = "PRESS_KEY")]
	PRESS_KEY,
    #[serde(rename = "SERVER_CERTIFICATE")]
	SERVER_CERTIFICATE,
    #[serde(rename = "SELECT_OPTION")]
	SELECT_OPTION,
    #[serde(rename = "STEP_TIMEOUT")]
	STEP_TIMEOUT,
    #[serde(rename = "SUB_TEST_NOT_PASSED")]
	SUB_TEST_NOT_PASSED,
    #[serde(rename = "TEST_TIMEOUT")]
	TEST_TIMEOUT,
    #[serde(rename = "TOO_MANY_HTTP_REQUESTS")]
	TOO_MANY_HTTP_REQUESTS,
    #[serde(rename = "UNAVAILABLE_BROWSER")]
	UNAVAILABLE_BROWSER,
    #[serde(rename = "UNKNOWN")]
	UNKNOWN,
    #[serde(rename = "UNSUPPORTED_AUTH_SCHEMA")]
	UNSUPPORTED_AUTH_SCHEMA,
    #[serde(rename = "UPLOAD_FILES_ELEMENT_TYPE")]
	UPLOAD_FILES_ELEMENT_TYPE,
    #[serde(rename = "UPLOAD_FILES_DIALOG")]
	UPLOAD_FILES_DIALOG,
    #[serde(rename = "UPLOAD_FILES_DYNAMIC_ELEMENT")]
	UPLOAD_FILES_DYNAMIC_ELEMENT,
    #[serde(rename = "UPLOAD_FILES_NAME")]
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

impl Default for SyntheticsBrowserTestFailureCode {
    fn default() -> SyntheticsBrowserTestFailureCode {
        Self::API_REQUEST_FAILURE
    }
}
