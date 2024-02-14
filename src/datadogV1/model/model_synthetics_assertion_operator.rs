// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsAssertionOperator {
    #[serde(rename = "contains")]
    CONTAINS,
    #[serde(rename = "doesNotContain")]
    DOES_NOT_CONTAIN,
    #[serde(rename = "is")]
    IS,
    #[serde(rename = "isNot")]
    IS_NOT,
    #[serde(rename = "lessThan")]
    LESS_THAN,
    #[serde(rename = "lessThanOrEqual")]
    LESS_THAN_OR_EQUAL,
    #[serde(rename = "moreThan")]
    MORE_THAN,
    #[serde(rename = "moreThanOrEqual")]
    MORE_THAN_OR_EQUAL,
    #[serde(rename = "matches")]
    MATCHES,
    #[serde(rename = "doesNotMatch")]
    DOES_NOT_MATCH,
    #[serde(rename = "validates")]
    VALIDATES,
    #[serde(rename = "isInMoreThan")]
    IS_IN_MORE_DAYS_THAN,
    #[serde(rename = "isInLessThan")]
    IS_IN_LESS_DAYS_THAN,
    #[serde(rename = "doesNotExist")]
    DOES_NOT_EXIST,
    #[serde(rename = "isUndefined")]
    IS_UNDEFINED,
}

impl ToString for SyntheticsAssertionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::CONTAINS => String::from("contains"),
            Self::DOES_NOT_CONTAIN => String::from("doesNotContain"),
            Self::IS => String::from("is"),
            Self::IS_NOT => String::from("isNot"),
            Self::LESS_THAN => String::from("lessThan"),
            Self::LESS_THAN_OR_EQUAL => String::from("lessThanOrEqual"),
            Self::MORE_THAN => String::from("moreThan"),
            Self::MORE_THAN_OR_EQUAL => String::from("moreThanOrEqual"),
            Self::MATCHES => String::from("matches"),
            Self::DOES_NOT_MATCH => String::from("doesNotMatch"),
            Self::VALIDATES => String::from("validates"),
            Self::IS_IN_MORE_DAYS_THAN => String::from("isInMoreThan"),
            Self::IS_IN_LESS_DAYS_THAN => String::from("isInLessThan"),
            Self::DOES_NOT_EXIST => String::from("doesNotExist"),
            Self::IS_UNDEFINED => String::from("isUndefined"),
        }
    }
}
