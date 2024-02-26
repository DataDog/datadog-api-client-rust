// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsWarningType {
    #[serde(rename = "user_locator")]
    USER_LOCATOR,
}

impl ToString for SyntheticsWarningType {
    fn to_string(&self) -> String {
        match self {
            Self::USER_LOCATOR => String::from("user_locator"),
        }
    }
}
