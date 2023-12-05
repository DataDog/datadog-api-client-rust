// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective data container.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchServiceLevelObjective {
    /// A service level objective ID and attributes.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SearchServiceLevelObjectiveData>>,
}

impl SearchServiceLevelObjective {
    pub fn new() -> SearchServiceLevelObjective {
        SearchServiceLevelObjective { data: None }
    }
}