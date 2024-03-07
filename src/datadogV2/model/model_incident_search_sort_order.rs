// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IncidentSearchSortOrder {
    #[serde(rename = "created")]
    CREATED_ASCENDING,
    #[serde(rename = "-created")]
    CREATED_DESCENDING,
}

impl ToString for IncidentSearchSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_ASCENDING => String::from("created"),
            Self::CREATED_DESCENDING => String::from("-created"),
        }
    }
}
