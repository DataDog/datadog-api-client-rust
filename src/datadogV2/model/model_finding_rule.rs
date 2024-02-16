// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The rule that triggered this finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindingRule {
    /// The ID of the rule that triggered this finding.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the rule that triggered this finding.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl FindingRule {
    pub fn new() -> FindingRule {
        FindingRule {
            id: None,
            name: None,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for FindingRule {
    fn default() -> Self {
        Self::new()
    }
}
