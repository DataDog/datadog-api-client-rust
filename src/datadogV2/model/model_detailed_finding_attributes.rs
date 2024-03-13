// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API attributes of the detailed finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailedFindingAttributes {
    /// The evaluation of the finding.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// The date on which the evaluation for this finding changed (Unix ms).
    #[serde(rename = "evaluation_changed_at")]
    pub evaluation_changed_at: Option<i64>,
    /// The remediation message for this finding.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Information about the mute status of this finding.
    #[serde(rename = "mute")]
    pub mute: Option<crate::datadogV2::model::FindingMute>,
    /// The resource name of this finding.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// The resource configuration for this finding.
    #[serde(rename = "resource_configuration")]
    pub resource_configuration: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The date on which the resource was discovered (Unix ms).
    #[serde(rename = "resource_discovery_date")]
    pub resource_discovery_date: Option<i64>,
    /// The resource type of this finding.
    #[serde(rename = "resource_type")]
    pub resource_type: Option<String>,
    /// The rule that triggered this finding.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::FindingRule>,
    /// The status of the finding.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::FindingStatus>,
    /// The tags associated with this finding.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl DetailedFindingAttributes {
    pub fn new() -> DetailedFindingAttributes {
        DetailedFindingAttributes {
            evaluation: None,
            evaluation_changed_at: None,
            message: None,
            mute: None,
            resource: None,
            resource_configuration: None,
            resource_discovery_date: None,
            resource_type: None,
            rule: None,
            status: None,
            tags: None,
        }
    }

    pub fn evaluation(mut self, value: crate::datadogV2::model::FindingEvaluation) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn evaluation_changed_at(mut self, value: i64) -> Self {
        self.evaluation_changed_at = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn mute(mut self, value: crate::datadogV2::model::FindingMute) -> Self {
        self.mute = Some(value);
        self
    }

    pub fn resource(mut self, value: String) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn resource_configuration(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.resource_configuration = Some(value);
        self
    }

    pub fn resource_discovery_date(mut self, value: i64) -> Self {
        self.resource_discovery_date = Some(value);
        self
    }

    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::FindingRule) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::FindingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for DetailedFindingAttributes {
    fn default() -> Self {
        Self::new()
    }
}
