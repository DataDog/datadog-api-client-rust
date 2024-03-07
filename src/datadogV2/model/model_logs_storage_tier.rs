// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogsStorageTier {
    #[serde(rename = "indexes")]
    INDEXES,
    #[serde(rename = "online-archives")]
    ONLINE_ARCHIVES,
}
impl ToString for LogsStorageTier {
    fn to_string(&self) -> String {
        match self {
            Self::INDEXES => String::from("indexes"),
            Self::ONLINE_ARCHIVES => String::from("online-archives"),
        }
    }
}
