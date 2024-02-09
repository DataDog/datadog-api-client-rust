// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API relationship to an outcome, which returns the related rule id.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToOutcomeData {
    /// The unique ID for a scorecard rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for scorecard rules.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleType>,
}

impl RelationshipToOutcomeData {
    pub fn new() -> RelationshipToOutcomeData {
        RelationshipToOutcomeData {
            id: None,
            type_: None,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::RuleType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for RelationshipToOutcomeData {
    fn default() -> Self {
        Self::new()
    }
}
