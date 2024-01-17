// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the included rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesResponseIncludedItem {
    /// Details of a rule.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::OutcomesResponseIncludedRuleAttributes>>,
    /// The unique ID for a scorecard rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for scorecard rules.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::RuleType>,
}

impl OutcomesResponseIncludedItem {
    pub fn new() -> OutcomesResponseIncludedItem {
        OutcomesResponseIncludedItem {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for OutcomesResponseIncludedItem {
    fn default() -> Self {
        Self::new()
    }
}
