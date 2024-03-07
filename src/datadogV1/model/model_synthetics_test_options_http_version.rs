// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SyntheticsTestOptionsHTTPVersion {
    #[serde(rename = "http1")]
    HTTP1,
    #[serde(rename = "http2")]
    HTTP2,
    #[serde(rename = "any")]
    ANY,
}

impl ToString for SyntheticsTestOptionsHTTPVersion {
    fn to_string(&self) -> String {
        match self {
            Self::HTTP1 => String::from("http1"),
            Self::HTTP2 => String::from("http2"),
            Self::ANY => String::from("any"),
        }
    }
}
