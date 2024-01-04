// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBrowserErrorType {
    #[serde(rename = "network")]
    NETWORK,
    #[serde(rename = "js")]
    JS,
}

impl ToString for SyntheticsBrowserErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::NETWORK => String::from("network"),
            Self::JS => String::from("js"),
        }
    }
}
