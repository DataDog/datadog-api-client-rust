// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum APIKeysSort {
    #[serde(rename = "created_at")]
    CREATED_AT_ASCENDING,
    #[serde(rename = "-created_at")]
    CREATED_AT_DESCENDING,
    #[serde(rename = "last4")]
    LAST4_ASCENDING,
    #[serde(rename = "-last4")]
    LAST4_DESCENDING,
    #[serde(rename = "modified_at")]
    MODIFIED_AT_ASCENDING,
    #[serde(rename = "-modified_at")]
    MODIFIED_AT_DESCENDING,
    #[serde(rename = "name")]
    NAME_ASCENDING,
    #[serde(rename = "-name")]
    NAME_DESCENDING,
}

impl ToString for APIKeysSort {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_AT_ASCENDING => String::from("created_at"),
            Self::CREATED_AT_DESCENDING => String::from("-created_at"),
            Self::LAST4_ASCENDING => String::from("last4"),
            Self::LAST4_DESCENDING => String::from("-last4"),
            Self::MODIFIED_AT_ASCENDING => String::from("modified_at"),
            Self::MODIFIED_AT_DESCENDING => String::from("-modified_at"),
            Self::NAME_ASCENDING => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
        }
    }
}
