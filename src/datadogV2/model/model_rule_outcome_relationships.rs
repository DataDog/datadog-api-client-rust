// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API relationship to a scorecard rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleOutcomeRelationships {
    /// The JSON:API relationship to a scorecard outcome.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::RelationshipToOutcome>,
}

impl RuleOutcomeRelationships {
    pub fn new() -> RuleOutcomeRelationships {
        RuleOutcomeRelationships { rule: None }
    }

    pub fn rule(mut self, value: crate::datadogV2::model::RelationshipToOutcome) -> Self {
        self.rule = Some(value);
        self
    }
}

impl Default for RuleOutcomeRelationships {
    fn default() -> Self {
        Self::new()
    }
}
