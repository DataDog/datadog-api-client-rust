// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnMissingDataOption {
    #[serde(rename = "default")]
    DEFAULT,
    #[serde(rename = "show_no_data")]
    SHOW_NO_DATA,
    #[serde(rename = "show_and_notify_no_data")]
    SHOW_AND_NOTIFY_NO_DATA,
    #[serde(rename = "resolve")]
    RESOLVE,
}

impl ToString for OnMissingDataOption {
    fn to_string(&self) -> String {
        match self {
            Self::DEFAULT => String::from("default"),
            Self::SHOW_NO_DATA => String::from("show_no_data"),
            Self::SHOW_AND_NOTIFY_NO_DATA => String::from("show_and_notify_no_data"),
            Self::RESOLVE => String::from("resolve"),
        }
    }
}
