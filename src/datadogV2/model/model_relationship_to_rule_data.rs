// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship data for a rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToRuleData {
    /// Rule relationship data.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::RelationshipToRuleDataObject>>,
}

impl RelationshipToRuleData {
    pub fn new() -> RelationshipToRuleData {
        RelationshipToRuleData { data: None }
    }
}
impl Default for RelationshipToRuleData {
    fn default() -> Self {
        Self::new()
    }
}
