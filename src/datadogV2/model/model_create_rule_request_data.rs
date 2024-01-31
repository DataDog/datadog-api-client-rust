// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard create rule request data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRuleRequestData {
    /// Details of a rule.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RuleAttributes>,
    /// The JSON:API type for scorecard rules.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleType>,
}

impl CreateRuleRequestData {
    pub fn new() -> CreateRuleRequestData {
        CreateRuleRequestData {
            attributes: None,
            type_: None,
        }
    }

    pub fn with_attributes(&mut self, value: crate::datadogV2::model::RuleAttributes) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::RuleType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for CreateRuleRequestData {
    fn default() -> Self {
        Self::new()
    }
}
