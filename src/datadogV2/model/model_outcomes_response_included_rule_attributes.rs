// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of a rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesResponseIncludedRuleAttributes {
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The scorecard name to which this rule must belong.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: Option<String>,
}

impl OutcomesResponseIncludedRuleAttributes {
    pub fn new() -> OutcomesResponseIncludedRuleAttributes {
        OutcomesResponseIncludedRuleAttributes {
            name: None,
            scorecard_name: None,
        }
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_scorecard_name(&mut self, value: String) -> &mut Self {
        self.scorecard_name = Some(value);
        self
    }
}
impl Default for OutcomesResponseIncludedRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}
