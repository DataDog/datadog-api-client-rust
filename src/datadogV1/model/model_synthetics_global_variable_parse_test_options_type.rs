// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsGlobalVariableParseTestOptionsType {
    #[serde(rename = "http_body")]
    HTTP_BODY,
    #[serde(rename = "http_header")]
    HTTP_HEADER,
    #[serde(rename = "local_variable")]
    LOCAL_VARIABLE,
}

impl ToString for SyntheticsGlobalVariableParseTestOptionsType {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP_BODY => String::from("http_body"),
            Self::HTTP_HEADER => String::from("http_header"),
            Self::LOCAL_VARIABLE => String::from("local_variable"),
        }
    }
}
