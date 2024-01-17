// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single rule outcome.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesResponseDataItem {
    /// The JSON:API attributes for an outcome.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::OutcomesBatchResponseAttributes>>,
    /// The unique ID for a rule outcome.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API relationship to a scorecard rule.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::RuleOutcomeRelationships>>,
    /// The JSON:API type for an outcome.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OutcomeType>,
}

impl OutcomesResponseDataItem {
    pub fn new() -> OutcomesResponseDataItem {
        OutcomesResponseDataItem {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for OutcomesResponseDataItem {
    fn default() -> Self {
        Self::new()
    }
}
