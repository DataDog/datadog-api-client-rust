// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotifyEndType {
    #[serde(rename = "canceled")]
    CANCELED,
    #[serde(rename = "expired")]
    EXPIRED,
}

impl ToString for NotifyEndType {
    fn to_string(&self) -> String {
        match self {
            Self::CANCELED => String::from("canceled"),
            Self::EXPIRED => String::from("expired"),
        }
    }
}

impl Default for NotifyEndType {
    fn default() -> NotifyEndType {
        Self::CANCELED
    }
}