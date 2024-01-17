// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API relationship to a scorecard outcome.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToOutcome {
    /// The JSON:API relationship to an outcome, which returns the related rule id.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::RelationshipToOutcomeData>>,
}

impl RelationshipToOutcome {
    pub fn new() -> RelationshipToOutcome {
        RelationshipToOutcome { data: None }
    }
}
impl Default for RelationshipToOutcome {
    fn default() -> Self {
        Self::new()
    }
}
