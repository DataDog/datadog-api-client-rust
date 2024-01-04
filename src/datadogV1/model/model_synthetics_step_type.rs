// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsStepType {
    #[serde(rename = "assertCurrentUrl")]
    ASSERT_CURRENT_URL,
    #[serde(rename = "assertElementAttribute")]
    ASSERT_ELEMENT_ATTRIBUTE,
    #[serde(rename = "assertElementContent")]
    ASSERT_ELEMENT_CONTENT,
    #[serde(rename = "assertElementPresent")]
    ASSERT_ELEMENT_PRESENT,
    #[serde(rename = "assertEmail")]
    ASSERT_EMAIL,
    #[serde(rename = "assertFileDownload")]
    ASSERT_FILE_DOWNLOAD,
    #[serde(rename = "assertFromJavascript")]
    ASSERT_FROM_JAVASCRIPT,
    #[serde(rename = "assertPageContains")]
    ASSERT_PAGE_CONTAINS,
    #[serde(rename = "assertPageLacks")]
    ASSERT_PAGE_LACKS,
    #[serde(rename = "click")]
    CLICK,
    #[serde(rename = "extractFromJavascript")]
    EXTRACT_FROM_JAVASCRIPT,
    #[serde(rename = "extractVariable")]
    EXTRACT_VARIABLE,
    #[serde(rename = "goToEmailLink")]
    GO_TO_EMAIL_LINK,
    #[serde(rename = "goToUrl")]
    GO_TO_URL,
    #[serde(rename = "goToUrlAndMeasureTti")]
    GO_TO_URL_AND_MEASURE_TTI,
    #[serde(rename = "hover")]
    HOVER,
    #[serde(rename = "playSubTest")]
    PLAY_SUB_TEST,
    #[serde(rename = "pressKey")]
    PRESS_KEY,
    #[serde(rename = "refresh")]
    REFRESH,
    #[serde(rename = "runApiTest")]
    RUN_API_TEST,
    #[serde(rename = "scroll")]
    SCROLL,
    #[serde(rename = "selectOption")]
    SELECT_OPTION,
    #[serde(rename = "typeText")]
    TYPE_TEXT,
    #[serde(rename = "uploadFiles")]
    UPLOAD_FILES,
    #[serde(rename = "wait")]
    WAIT,
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
            Self::CLICK => String::from("click"),
            Self::EXTRACT_FROM_JAVASCRIPT => String::from("extractFromJavascript"),
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
        }
    }
}
