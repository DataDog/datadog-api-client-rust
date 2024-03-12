// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create rule response data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRuleResponseData {
    /// Details of a rule.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RuleAttributes>,
    /// The unique ID for a scorecard rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Scorecard create rule response relationship.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::RelationshipToRule>,
    /// The JSON:API type for scorecard rules.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleType>,
}

impl CreateRuleResponseData {
    pub fn new() -> CreateRuleResponseData {
        CreateRuleResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::RuleAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(mut self, value: crate::datadogV2::model::RelationshipToRule) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::RuleType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for CreateRuleResponseData {
    fn default() -> Self {
        Self::new()
    }
}
