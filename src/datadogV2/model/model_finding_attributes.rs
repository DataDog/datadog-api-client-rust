// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API attributes of the finding.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindingAttributes {
    /// The evaluation of the finding.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// The date on which the evaluation for this finding changed (Unix ms).
    #[serde(rename = "evaluation_changed_at")]
    pub evaluation_changed_at: Option<i64>,
    /// Information about the mute status of this finding.
    #[serde(rename = "mute")]
    pub mute: Option<crate::datadogV2::model::FindingMute>,
    /// The resource name of this finding.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
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

impl FindingAttributes {
    pub fn new() -> FindingAttributes {
        FindingAttributes {
            evaluation: None,
            evaluation_changed_at: None,
            mute: None,
            resource: None,
            resource_discovery_date: None,
            resource_type: None,
            rule: None,
            status: None,
            tags: None,
        }
    }

    pub fn with_evaluation(
        &mut self,
        value: crate::datadogV2::model::FindingEvaluation,
    ) -> &mut Self {
        self.evaluation = Some(value);
        self
    }

    pub fn with_evaluation_changed_at(&mut self, value: i64) -> &mut Self {
        self.evaluation_changed_at = Some(value);
        self
    }

    pub fn with_mute(&mut self, value: crate::datadogV2::model::FindingMute) -> &mut Self {
        self.mute = Some(value);
        self
    }

    pub fn with_resource(&mut self, value: String) -> &mut Self {
        self.resource = Some(value);
        self
    }

    pub fn with_resource_discovery_date(&mut self, value: i64) -> &mut Self {
        self.resource_discovery_date = Some(value);
        self
    }

    pub fn with_resource_type(&mut self, value: String) -> &mut Self {
        self.resource_type = Some(value);
        self
    }

    pub fn with_rule(&mut self, value: crate::datadogV2::model::FindingRule) -> &mut Self {
        self.rule = Some(value);
        self
    }

    pub fn with_status(&mut self, value: crate::datadogV2::model::FindingStatus) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
impl Default for FindingAttributes {
    fn default() -> Self {
        Self::new()
    }
}
