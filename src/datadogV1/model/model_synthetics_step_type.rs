// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsStepType {
    ASSERT_CURRENT_URL,
    ASSERT_ELEMENT_ATTRIBUTE,
    ASSERT_ELEMENT_CONTENT,
    ASSERT_ELEMENT_PRESENT,
    ASSERT_EMAIL,
    ASSERT_FILE_DOWNLOAD,
    ASSERT_FROM_JAVASCRIPT,
    ASSERT_PAGE_CONTAINS,
    ASSERT_PAGE_LACKS,
    ASSERT_REQUESTS,
    CLICK,
    EXTRACT_FROM_JAVASCRIPT,
    EXTRACT_FROM_EMAIL_BODY,
    EXTRACT_VARIABLE,
    GO_TO_EMAIL_LINK,
    GO_TO_URL,
    GO_TO_URL_AND_MEASURE_TTI,
    HOVER,
    PLAY_SUB_TEST,
    PRESS_KEY,
    REFRESH,
    RUN_API_TEST,
    SCROLL,
    SELECT_OPTION,
    TYPE_TEXT,
    UPLOAD_FILES,
    WAIT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsStepType {
    fn to_string(&self) -> String {
        match self {
            Self::ASSERT_CURRENT_URL => String::from("assertCurrentUrl"),
            Self::ASSERT_ELEMENT_ATTRIBUTE => String::from("assertElementAttribute"),
            Self::ASSERT_ELEMENT_CONTENT => String::from("assertElementContent"),
            Self::ASSERT_ELEMENT_PRESENT => String::from("assertElementPresent"),
            Self::ASSERT_EMAIL => String::from("assertEmail"),
            Self::ASSERT_FILE_DOWNLOAD => String::from("assertFileDownload"),
            Self::ASSERT_FROM_JAVASCRIPT => String::from("assertFromJavascript"),
            Self::ASSERT_PAGE_CONTAINS => String::from("assertPageContains"),
            Self::ASSERT_PAGE_LACKS => String::from("assertPageLacks"),
            Self::ASSERT_REQUESTS => String::from("assertRequests"),
            Self::CLICK => String::from("click"),
            Self::EXTRACT_FROM_JAVASCRIPT => String::from("extractFromJavascript"),
            Self::EXTRACT_FROM_EMAIL_BODY => String::from("extractFromEmailBody"),
            Self::EXTRACT_VARIABLE => String::from("extractVariable"),
            Self::GO_TO_EMAIL_LINK => String::from("goToEmailLink"),
            Self::GO_TO_URL => String::from("goToUrl"),
            Self::GO_TO_URL_AND_MEASURE_TTI => String::from("goToUrlAndMeasureTti"),
            Self::HOVER => String::from("hover"),
            Self::PLAY_SUB_TEST => String::from("playSubTest"),
            Self::PRESS_KEY => String::from("pressKey"),
            Self::REFRESH => String::from("refresh"),
            Self::RUN_API_TEST => String::from("runApiTest"),
            Self::SCROLL => String::from("scroll"),
            Self::SELECT_OPTION => String::from("selectOption"),
            Self::TYPE_TEXT => String::from("typeText"),
            Self::UPLOAD_FILES => String::from("uploadFiles"),
            Self::WAIT => String::from("wait"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsStepType {
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

impl<'de> Deserialize<'de> for SyntheticsStepType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "assertCurrentUrl" => Self::ASSERT_CURRENT_URL,
            "assertElementAttribute" => Self::ASSERT_ELEMENT_ATTRIBUTE,
            "assertElementContent" => Self::ASSERT_ELEMENT_CONTENT,
            "assertElementPresent" => Self::ASSERT_ELEMENT_PRESENT,
            "assertEmail" => Self::ASSERT_EMAIL,
            "assertFileDownload" => Self::ASSERT_FILE_DOWNLOAD,
            "assertFromJavascript" => Self::ASSERT_FROM_JAVASCRIPT,
            "assertPageContains" => Self::ASSERT_PAGE_CONTAINS,
            "assertPageLacks" => Self::ASSERT_PAGE_LACKS,
            "assertRequests" => Self::ASSERT_REQUESTS,
            "click" => Self::CLICK,
            "extractFromJavascript" => Self::EXTRACT_FROM_JAVASCRIPT,
            "extractFromEmailBody" => Self::EXTRACT_FROM_EMAIL_BODY,
            "extractVariable" => Self::EXTRACT_VARIABLE,
            "goToEmailLink" => Self::GO_TO_EMAIL_LINK,
            "goToUrl" => Self::GO_TO_URL,
            "goToUrlAndMeasureTti" => Self::GO_TO_URL_AND_MEASURE_TTI,
            "hover" => Self::HOVER,
            "playSubTest" => Self::PLAY_SUB_TEST,
            "pressKey" => Self::PRESS_KEY,
            "refresh" => Self::REFRESH,
            "runApiTest" => Self::RUN_API_TEST,
            "scroll" => Self::SCROLL,
            "selectOption" => Self::SELECT_OPTION,
            "typeText" => Self::TYPE_TEXT,
            "uploadFiles" => Self::UPLOAD_FILES,
            "wait" => Self::WAIT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
