// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of a rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleAttributes {
    /// The scorecard name to which this rule must belong.
    #[deprecated]
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Creation time of the rule outcome.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Defines if the rule is a custom rule.
    #[serde(rename = "custom")]
    pub custom: Option<bool>,
    /// Explanation of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// If enabled, the rule is calculated as part of the score.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Time of the last rule outcome modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Owner of the rule.
    #[serde(rename = "owner")]
    pub owner: Option<String>,
    /// The scorecard name to which this rule must belong.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: Option<String>,
}

impl RuleAttributes {
    pub fn new() -> RuleAttributes {
        #[allow(deprecated)]
        RuleAttributes {
            category: None,
            created_at: None,
            custom: None,
            description: None,
            enabled: None,
            modified_at: None,
            name: None,
            owner: None,
            scorecard_name: None,
        }
    }
}
