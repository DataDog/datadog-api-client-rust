// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard create rule response relationship.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToRule {
    /// Relationship data for a rule.
    #[serde(rename = "scorecard")]
    pub scorecard: Option<Box<crate::datadogV2::model::RelationshipToRuleData>>,
}

impl RelationshipToRule {
    pub fn new() -> RelationshipToRule {
        RelationshipToRule { scorecard: None }
    }
}
