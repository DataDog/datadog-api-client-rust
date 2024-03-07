// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SyntheticsCheckType {
    #[serde(rename = "equals")]
    EQUALS,
    #[serde(rename = "notEquals")]
    NOT_EQUALS,
    #[serde(rename = "contains")]
    CONTAINS,
    #[serde(rename = "notContains")]
    NOT_CONTAINS,
    #[serde(rename = "startsWith")]
    STARTS_WITH,
    #[serde(rename = "notStartsWith")]
    NOT_STARTS_WITH,
    #[serde(rename = "greater")]
    GREATER,
    #[serde(rename = "lower")]
    LOWER,
    #[serde(rename = "greaterEquals")]
    GREATER_EQUALS,
    #[serde(rename = "lowerEquals")]
    LOWER_EQUALS,
    #[serde(rename = "matchRegex")]
    MATCH_REGEX,
    #[serde(rename = "between")]
    BETWEEN,
    #[serde(rename = "isEmpty")]
    IS_EMPTY,
    #[serde(rename = "notIsEmpty")]
    NOT_IS_EMPTY,
}
impl ToString for SyntheticsCheckType {
    fn to_string(&self) -> String {
        match self {
            Self::EQUALS => String::from("equals"),
            Self::NOT_EQUALS => String::from("notEquals"),
            Self::CONTAINS => String::from("contains"),
            Self::NOT_CONTAINS => String::from("notContains"),
            Self::STARTS_WITH => String::from("startsWith"),
            Self::NOT_STARTS_WITH => String::from("notStartsWith"),
            Self::GREATER => String::from("greater"),
            Self::LOWER => String::from("lower"),
            Self::GREATER_EQUALS => String::from("greaterEquals"),
            Self::LOWER_EQUALS => String::from("lowerEquals"),
            Self::MATCH_REGEX => String::from("matchRegex"),
            Self::BETWEEN => String::from("between"),
            Self::IS_EMPTY => String::from("isEmpty"),
            Self::NOT_IS_EMPTY => String::from("notIsEmpty"),
        }
    }
}
