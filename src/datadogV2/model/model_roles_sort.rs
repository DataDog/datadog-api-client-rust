// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RolesSort {
    #[serde(rename = "name")]
    NAME_ASCENDING,
    #[serde(rename = "-name")]
    NAME_DESCENDING,
    #[serde(rename = "modified_at")]
    MODIFIED_AT_ASCENDING,
    #[serde(rename = "-modified_at")]
    MODIFIED_AT_DESCENDING,
    #[serde(rename = "user_count")]
    USER_COUNT_ASCENDING,
    #[serde(rename = "-user_count")]
    USER_COUNT_DESCENDING,
}
impl ToString for RolesSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME_ASCENDING => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::MODIFIED_AT_ASCENDING => String::from("modified_at"),
            Self::MODIFIED_AT_DESCENDING => String::from("-modified_at"),
            Self::USER_COUNT_ASCENDING => String::from("user_count"),
            Self::USER_COUNT_DESCENDING => String::from("-user_count"),
        }
    }
}
