// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SpansSortOrder {
    #[serde(rename = "asc")]
    ASCENDING,
    #[serde(rename = "desc")]
    DESCENDING,
}

impl ToString for SpansSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::ASCENDING => String::from("asc"),
            Self::DESCENDING => String::from("desc"),
        }
    }
}
