// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IncidentSearchResultsType {
    #[serde(rename = "incidents_search_results")]
    INCIDENTS_SEARCH_RESULTS,
}

impl ToString for IncidentSearchResultsType {
    fn to_string(&self) -> String {
        match self {
            Self::INCIDENTS_SEARCH_RESULTS => String::from("incidents_search_results"),
        }
    }
}
