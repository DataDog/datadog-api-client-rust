// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestRequestBodyType {
    #[serde(rename = "text/plain")]
    TEXT_PLAIN,
    #[serde(rename = "application/json")]
    APPLICATION_JSON,
    #[serde(rename = "text/xml")]
    TEXT_XML,
    #[serde(rename = "text/html")]
    TEXT_HTML,
    #[serde(rename = "application/x-www-form-urlencoded")]
    APPLICATION_X_WWW_FORM_URLENCODED,
    #[serde(rename = "graphql")]
    GRAPHQL,
}

impl ToString for SyntheticsTestRequestBodyType {
    fn to_string(&self) -> String {
        match self {
            Self::TEXT_PLAIN => String::from("text/plain"),
            Self::APPLICATION_JSON => String::from("application/json"),
            Self::TEXT_XML => String::from("text/xml"),
            Self::TEXT_HTML => String::from("text/html"),
            Self::APPLICATION_X_WWW_FORM_URLENCODED => {
                String::from("application/x-www-form-urlencoded")
            }
            Self::GRAPHQL => String::from("graphql"),
        }
    }
}
