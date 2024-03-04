// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Rule relationship data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToRuleDataObject {
    /// The unique ID for a scorecard.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The JSON:API type for scorecard.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScorecardType>,
}

impl RelationshipToRuleDataObject {
    pub fn new() -> RelationshipToRuleDataObject {
        RelationshipToRuleDataObject {
            id: None,
            type_: None,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ScorecardType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for RelationshipToRuleDataObject {
    fn default() -> Self {
        Self::new()
    }
}
